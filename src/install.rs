// zippy install — download, verify, install, configure

use crate::{platform, ui};
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

const ZETA_REPO: &str = "murphsicles/zeta";
const ZETA_LATEST: &str = "v1.0.18";


/// Run the full installation flow.
pub async fn run(
    version: Option<&str>,
    prefix: Option<&str>,
    no_path: bool,
) -> Result<(), String> {
    ui::print_logo();

    let version = version.unwrap_or(ZETA_LATEST);
    if !version.starts_with('v') {
        return Err("Version must start with 'v' (e.g. v1.0.18)".to_string());
    }

    // ── Step 1: Platform detection ──────────────────────────────────────────
    ui::step("Detecting platform...");
    let target = platform::detect()?;
    let install_root = prefix
        .map(PathBuf::from)
        .unwrap_or_else(|| platform::zeta_root().unwrap());
    let bin_dir = install_root.join("bin");
    ui::step_ok(&format!("{}", target));

    // ── Step 2: Create directories ──────────────────────────────────────────
    ui::step("Creating directories...");
    std::fs::create_dir_all(&bin_dir)
        .map_err(|e| format!("Failed to create {}: {}", bin_dir.display(), e))?;
    std::fs::create_dir_all(install_root.join("versions"))
        .map_err(|e| format!("Failed to create versions dir: {}", e))?;
    ui::step_ok(&format!("~/.zeta/ ({})", install_root.display()));

    // ── Step 3: Download zetac ──────────────────────────────────────────────
    let download_url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        ZETA_REPO, version, target.binary_name
    );
    let dest_path = bin_dir.join("zetac");

    ui::step("Downloading zetac...");
    download_file(&download_url, &dest_path, Some(target.binary_name.as_str())).await?;
    ui::step_ok(&format!("{} ({})", target.binary_name, version));

    // ── Step 4: Make executable ─────────────────────────────────────────────
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&dest_path, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("Failed to set executable bit: {}", e))?;
    }
    #[cfg(windows)]
    {
        // Ensure .exe extension on Windows
        let exe_path = bin_dir.join("zetac.exe");
        std::fs::rename(&dest_path, &exe_path)
            .map_err(|e| format!("Failed to rename: {}", e))?;
    }

    // ── Step 5: Verify installation ─────────────────────────────────────────
    ui::step("Verifying installation...");
    let zetac_path = if cfg!(windows) { bin_dir.join("zetac.exe") } else { dest_path };
    if !zetac_path.exists() {
        return Err("Installation failed: zetac binary not found.".to_string());
    }
    ui::step_ok("zetac is ready");

    // ── Step 6: Configure shell PATH ────────────────────────────────────────
    if !no_path {
        configure_path(&bin_dir)?;
    }

    // ── Step 7: Save version metadata ───────────────────────────────────────
    save_metadata(&install_root, version)?;

    // ── Done ────────────────────────────────────────────────────────────────
    ui::print_success_box(&[
        &format!("✓ Zeta {} installed!", &version[1..]),
        "",
        "  zetac --version",
        "  zetac --zorb search nour",
        "",
        "  z-lang.org  ·  zorbs.io/docs",
        "  Join the mission → murphsicles/proton.me",
    ]);

    println!("  {}z-lang.org{}  ·  {}zorbs.io/docs{}", ui::Zeta::ACCENT, ui::Zeta::RESET, ui::Zeta::MUTED, ui::Zeta::RESET);
    println!();

    Ok(())
}

/// Check the status of an existing installation.
pub async fn status() -> Result<(), String> {
    ui::divider();
    println!("  {}Zeta Status{}", ui::Zeta::BOLD, ui::Zeta::RESET);
    ui::divider();

    let root = platform::zeta_root()?;
    let bin_dir = platform::zeta_bin_dir()?;
    let zetac = if cfg!(windows) { bin_dir.join("zetac.exe") } else { bin_dir.join("zetac") };

    match std::fs::metadata(&zetac) {
        Ok(_) => {
            let meta = load_metadata(&root).unwrap_or_default();
            let ver = meta.get("version").cloned().unwrap_or_else(|| "unknown".to_string());
            println!("  {}✓{} Zeta {} is installed", ui::Zeta::ACCENT, ui::Zeta::RESET, ver);
            println!("  {}  Location:{}  {}", ui::Zeta::MUTED, ui::Zeta::RESET, zetac.display());
            println!("  {}  Size:{}     {} MB", ui::Zeta::MUTED, ui::Zeta::RESET, zetac_size(&zetac));
        }
        Err(_) => {
            println!("  {}Zeta is not installed.{}", ui::Zeta::MUTED, ui::Zeta::RESET);
            println!("  Run {}zippy install{} to get started.", ui::Zeta::ACCENT, ui::Zeta::RESET);
        }
    }

    ui::divider();
    Ok(())
}

/// List all installed Zeta versions.
pub async fn list_versions() -> Result<(), String> {
    let root = platform::zeta_root()?;
    let versions_dir = root.join("versions");

    if !versions_dir.exists() {
        println!("  {}No versions installed.{}", ui::Zeta::MUTED, ui::Zeta::RESET);
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&versions_dir)
        .map_err(|e| format!("Failed to read versions: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    println!("  {}Installed versions:{}", ui::Zeta::BOLD, ui::Zeta::RESET);
    for entry in &entries {
        println!("  {} {}", ui::Zeta::ACCENT, entry.file_name().to_string_lossy());
    }

    Ok(())
}

/// Set the default Zeta version.
pub async fn set_default(version: &str) -> Result<(), String> {
    let root = platform::zeta_root()?;
    let meta = std::collections::HashMap::from([
        ("version".to_string(), format!("v{}", version.trim_start_matches('v'))),
    ]);
    let json = serde_json::to_string_pretty(&meta)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
    tokio::fs::write(root.join("zeta.json"), json).await
        .map_err(|e| format!("Failed to write metadata: {}", e))?;
    println!("  {}Default version set to {}v{}{}", ui::Zeta::ACCENT, ui::Zeta::BOLD, version, ui::Zeta::RESET);
    Ok(())
}

/// Uninstall Zeta.
pub async fn uninstall(force: bool) -> Result<(), String> {
    let root = platform::zeta_root()?;

    if !force {
        println!("  {}This will remove Zeta from:{}{} {}", ui::Zeta::ACCENT, ui::Zeta::RESET, ui::Zeta::BOLD, root.display());
        print!("  Continue? [y/N] ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("  {}Aborted.{}", ui::Zeta::MUTED, ui::Zeta::RESET);
            return Ok(());
        }
    }

    if root.exists() {
        std::fs::remove_dir_all(&root)
            .map_err(|e| format!("Failed to remove {}: {}", root.display(), e))?;
        println!("  {}✓ Zeta removed.{}", ui::Zeta::ACCENT, ui::Zeta::RESET);

        // Try to clean up PATH entries
        remove_path_entries()?;
    } else {
        println!("  {}Zeta is not installed.{}", ui::Zeta::MUTED, ui::Zeta::RESET);
    }

    Ok(())
}

/// Run diagnostics.
pub async fn doctor() -> Result<(), String> {
    ui::divider();
    println!("  {}Zeta Doctor{}", ui::Zeta::BOLD, ui::Zeta::RESET);
    ui::divider();

    let mut all_good = true;

    // Check 1: Is zetac in PATH?
    match platform::which("zetac") {
        Some(path) => {
            println!("  {}✓{} zetac found: {}", ui::Zeta::ACCENT, ui::Zeta::RESET, path.display());
        }
        None => {
            println!("  {}✗{} zetac not found in PATH", "\x1b[38;2;239;68;68m", ui::Zeta::RESET);
            println!("  {}  Run 'zippy install' or add ~/.zeta/bin to your PATH{}", ui::Zeta::MUTED, ui::Zeta::RESET);
            all_good = false;
        }
    }

    // Check 2: Is Zeta root structure intact?
    let root = platform::zeta_root().unwrap_or_default();
    if root.join("bin").join("zetac").exists() || root.join("bin").join("zetac.exe").exists() {
        println!("  {}✓{} Zeta root found: {}", ui::Zeta::ACCENT, ui::Zeta::RESET, root.display());
    } else {
        println!("  {}✗{} Zeta root missing: {}", "\x1b[38;2;239;68;68m", ui::Zeta::RESET, root.display());
        all_good = false;
    }

    // Check 3: Is ~/.zeta/bin in PATH?
    let bin_dir = platform::zeta_bin_dir().unwrap_or_default();
    let path_var = std::env::var("PATH").unwrap_or_default();
    if path_var.contains(bin_dir.to_str().unwrap_or("")) {
        println!("  {}✓{} {} in PATH", ui::Zeta::ACCENT, ui::Zeta::RESET, bin_dir.display());
    } else {
        println!("  {}✗{} {} not in PATH", "\x1b[38;2;239;68;68m", ui::Zeta::RESET, bin_dir.display());
        println!("  {}  Add 'export PATH=\"$HOME/.zeta/bin:$PATH\"' to your shell profile{}", ui::Zeta::MUTED, ui::Zeta::RESET);
        all_good = false;
    }

    // Check 4: Shell config
    match platform::detect_shell() {
        Some(shell) => println!("  {}✓{} Shell detected: {}", ui::Zeta::ACCENT, ui::Zeta::RESET, shell),
        None => println!("  {}ℹ{} Could not detect shell", ui::Zeta::MUTED, ui::Zeta::RESET),
    }

    ui::divider();
    if all_good {
        println!("  {}✓ Everything looks good!{}", ui::Zeta::ACCENT, ui::Zeta::RESET);
    } else {
        println!("  {}ℹ Some issues found. Run 'zippy install' to fix.{}", ui::Zeta::MUTED, ui::Zeta::RESET);
    }
    println!();

    Ok(())
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Download a file with progress bar.
async fn download_file(url: &str, dest: &Path, _label: Option<&str>) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .user_agent("zippy/0.1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Download failed ({}): {}", status, body));
    }

    let total_size = response
        .content_length()
        .unwrap_or(0);

    // Create progress bar
    let pb = if total_size > 0 {
        let pb = ui::make_progress(total_size);
        pb
    } else {
        ui::make_spinner("Downloading...")
    };

    // Stream download
    let mut file = tokio::fs::File::create(dest)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Download error: {}", e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Write error: {}", e))?;
        downloaded += chunk.len() as u64;
        if total_size > 0 {
            pb.set_position(downloaded);
            pb.set_message(format!("{} / {}", format_size(downloaded), format_size(total_size)));
        }
    }

    pb.finish_and_clear();
    Ok(())
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}

/// Configure shell PATH by adding Zeta bin directory.
fn configure_path(bin_dir: &Path) -> Result<(), String> {
    let bin_str = bin_dir.to_str().ok_or("Invalid bin directory path")?;

    let shell = platform::detect_shell().unwrap_or_else(|| "bash".to_string());
    let rc_file = match shell.as_str() {
        "zsh" => platform::home_dir()?.join(".zshrc"),
        "bash" => platform::home_dir()?.join(".bashrc"),
        "fish" => platform::home_dir()?.join(".config/fish/config.fish"),
        _ => return Ok(()),
    };

    // Check if already in PATH
    let rc_content = std::fs::read_to_string(&rc_file).unwrap_or_default();
    if rc_content.contains(bin_str) {
        ui::info("Zeta bin already in PATH");
        return Ok(());
    }

    let export_line = match shell.as_str() {
        "fish" => format!("set -gx PATH {} $PATH\n", bin_str),
        _ => format!("\n# Added by zippy (Zeta installer)\nexport PATH=\"{}/.zeta/bin:$PATH\"\n",
            std::env::var("HOME").unwrap_or_default()),
    };

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&rc_file)
        .map_err(|e| format!("Failed to open {}: {}", rc_file.display(), e))?;

    use std::io::Write;
    file.write_all(export_line.as_bytes())
        .map_err(|e| format!("Failed to write to {}: {}", rc_file.display(), e))?;

    ui::step_ok(&format!("Configured {}", rc_file.display()));

    // Try to also update the current session
    let export_cmd = format!("export PATH=\"{}\":$PATH", bin_str);
    ui::info(&format!("Run: {}source {}{} or {}", ui::Zeta::ACCENT, rc_file.display(), ui::Zeta::RESET, export_cmd));

    Ok(())
}

/// Remove PATH entries from shell config on uninstall.
fn remove_path_entries() -> Result<(), String> {
    let shell = platform::detect_shell().unwrap_or_else(|| "bash".to_string());
    let rc_file = match shell.as_str() {
        "zsh" => platform::home_dir()?.join(".zshrc"),
        "bash" => platform::home_dir()?.join(".bashrc"),
        _ => return Ok(()),
    };

    if !rc_file.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&rc_file)
        .map_err(|e| format!("Failed to read {}: {}", rc_file.display(), e))?;

    let new_content: Vec<&str> = content
        .lines()
        .filter(|l| !l.contains(".zeta/bin") && !l.contains("zippy"))
        .collect();

    let new_content = new_content.join("\n");
    std::fs::write(&rc_file, new_content)
        .map_err(|e| format!("Failed to update {}: {}", rc_file.display(), e))?;

    Ok(())
}

/// Save installation metadata.
fn save_metadata(root: &Path, version: &str) -> Result<(), String> {
    use std::collections::HashMap;
    let meta = HashMap::from([
        ("version".to_string(), version.to_string()),
        ("installer".to_string(), "zippy".to_string()),
    ]);
    let json = serde_json::to_string_pretty(&meta)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
    std::fs::write(root.join("zeta.json"), json)
        .map_err(|e| format!("Failed to write metadata: {}", e))
}

/// Load installation metadata.
fn load_metadata(root: &Path) -> Option<std::collections::HashMap<String, String>> {
    let path = root.join("zeta.json");
    if !path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn zetac_size(path: &Path) -> String {
    if let Ok(meta) = std::fs::metadata(path) {
        format_size(meta.len())
    } else {
        "unknown".to_string()
    }
}

// zippy platform detection — OS, architecture, binary target names

use std::fmt;

/// Supported installation targets
#[derive(Debug, Clone, PartialEq)]
pub struct Target {
    pub os: Os,
    pub arch: Arch,
    /// Download filename on GitHub releases (e.g. "zetac-linux-x64")
    pub binary_name: String,
    /// Display label (e.g. "Linux x86_64")
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Os {
    Linux,
    Macos,
    Windows,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arch {
    X86_64,
    Aarch64,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

/// Detect the current platform and return the matching Zeta target.
pub fn detect() -> Result<Target, String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match (os, arch) {
        ("linux", "x86_64") => Ok(Target {
            os: Os::Linux,
            arch: Arch::X86_64,
            binary_name: "zetac-linux-x64".to_string(),
            label: "Linux x86_64".to_string(),
        }),
        ("linux", "aarch64") => Ok(Target {
            os: Os::Linux,
            arch: Arch::Aarch64,
            binary_name: "zetac-linux-arm64".to_string(),
            label: "Linux ARM64".to_string(),
        }),
        ("macos", "x86_64") => Ok(Target {
            os: Os::Macos,
            arch: Arch::X86_64,
            binary_name: "zetac-macos-x64".to_string(),
            label: "macOS x86_64".to_string(),
        }),
        ("macos", "aarch64") => Ok(Target {
            os: Os::Macos,
            arch: Arch::Aarch64,
            binary_name: "zetac-macos-arm64".to_string(),
            label: "macOS Apple Silicon".to_string(),
        }),
        ("windows", "x86_64") => Ok(Target {
            os: Os::Windows,
            arch: Arch::X86_64,
            binary_name: "zetac-windows-x64.exe".to_string(),
            label: "Windows x86_64".to_string(),
        }),
        ("windows", "aarch64") => Ok(Target {
            os: Os::Windows,
            arch: Arch::Aarch64,
            binary_name: "zetac-windows-arm64.exe".to_string(),
            label: "Windows ARM64".to_string(),
        }),
        _ => Err(format!(
            "Unsupported platform: {}-{}.\nZeta currently supports: Linux x86_64 (more coming soon).",
            os, arch
        )),
    }
}

/// Get the user's home directory.
pub fn home_dir() -> Result<std::path::PathBuf, String> {
    dirs::home_dir().ok_or_else(|| "Could not detect home directory.".to_string())
}

/// Get the Zeta install root (~/.zeta).
pub fn zeta_root() -> Result<std::path::PathBuf, String> {
    Ok(home_dir()?.join(".zeta"))
}

/// Get the bin directory within Zeta root.
pub fn zeta_bin_dir() -> Result<std::path::PathBuf, String> {
    Ok(zeta_root()?.join("bin"))
}

/// Detect the user's shell for PATH configuration.
pub fn detect_shell() -> Option<String> {
    // Check SHELL env var
    if let Ok(shell) = std::env::var("SHELL") {
        let shell = shell.trim().to_lowercase();
        if shell.contains("zsh") { return Some("zsh".to_string()); }
        if shell.contains("bash") { return Some("bash".to_string()); }
        if shell.contains("fish") { return Some("fish".to_string()); }
    }
    // Fallback: check what's available
    for candidate in &["zsh", "bash", "fish"] {
        if which(candidate).is_some() {
            return Some(candidate.to_string());
        }
    }
    None
}

pub fn which(name: &str) -> Option<std::path::PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths).find_map(|dir| {
            let full = dir.join(name);
            if full.is_file() { Some(full) } else { None }
        })
    })
}

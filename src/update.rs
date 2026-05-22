// zippy update — self-update and Zeta version management

use crate::{install, ui, platform};

/// Update Zeta to the latest (or specified) version.
pub async fn run(version: Option<&str>) -> Result<(), String> {
    ui::print_logo();

    let bin_dir = platform::zeta_bin_dir()?;
    let current_path = if cfg!(windows) { bin_dir.join("zetac.exe") } else { bin_dir.join("zetac") };

    // Check current installation
    if !current_path.exists() {
        return Err("Zeta is not installed. Run 'zippy install' first.".to_string());
    }

    let target_version = version.unwrap_or("v1.0.18");

    ui::step(&format!("Updating Zeta to {}...", target_version));

    // Re-use the install flow but with force flag
    install::run(Some(target_version), None, false).await?;

    // Update metadata
    install::set_default(target_version).await?;

    ui::print_success_box(&[
        &format!("✓ Updated to Zeta {}!", &target_version[1..]),
        "",
        "  zetac --version",
    ]);

    Ok(())
}

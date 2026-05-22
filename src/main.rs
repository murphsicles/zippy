// zippy — Zeta toolchain installer
// Fast, beautiful, zero-friction. One command, all platforms.

mod ui;
mod install;
mod update;
mod platform;

use clap::{Parser, Subcommand};
use std::process;

/// Zippy — Install Zeta in one command.
#[derive(Parser)]
#[command(
    name = "zippy",
    version,
    about = "⚡ Zeta toolchain installer",
    long_about = "Install, update, and manage the Zeta compiler.\n\n  curl zippy.sh | sh\n  zippy install\n  zippy update"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install Zeta (default action)
    Install {
        /// Version to install (e.g. "1.0.18", defaults to latest)
        #[arg(short, long)]
        version: Option<String>,

        /// Install directory (defaults to ~/.zeta)
        #[arg(short, long)]
        prefix: Option<String>,

        /// Skip shell PATH configuration
        #[arg(long)]
        no_path: bool,
    },

    /// Update Zeta to the latest version
    Update {
        /// Target version to update to
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Check current installation status
    Status,

    /// List installed versions
    List,

    /// Set default Zeta version
    Default {
        #[arg()]
        version: String,
    },

    /// Remove Zeta installation
    Uninstall {
        /// Non-interactive
        #[arg(long)]
        force: bool,
    },

    /// Run diagnostics and fix issues
    Doctor,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Install { version, prefix, no_path } => {
            install::run(version.as_deref(), prefix.as_deref(), *no_path).await
        }
        Commands::Update { version } => {
            update::run(version.as_deref()).await
        }
        Commands::Status => {
            install::status().await
        }
        Commands::List => {
            install::list_versions().await
        }
        Commands::Default { version } => {
            install::set_default(version).await
        }
        Commands::Uninstall { force } => {
            install::uninstall(*force).await
        }
        Commands::Doctor => {
            install::doctor().await
        }
    };

    if let Err(e) = result {
        ui::error(&format!("{}", e));
        process::exit(1);
    }
}



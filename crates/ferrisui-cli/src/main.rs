use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod config;
mod templates;

#[derive(Parser, Debug)]
#[command(
    name = "ferrisui",
    bin_name = "ferrisui",
    author = "FerrisUI Team",
    version,
    about = "🚀 Developer CLI for the FerrisUI GPU-accelerated UI framework",
    long_about = "Scaffold new desktop projects, inspect graphics & WGPU hardware capabilities, run with file-watching, and package production bundles."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Accommodates `cargo ferrisui <cmd>` where Cargo passes "ferrisui" as the first argument
#[derive(Parser, Debug)]
#[command(name = "cargo")]
struct CargoCli {
    #[command(subcommand)]
    command: CargoSubcommand,
}

#[derive(Subcommand, Debug)]
enum CargoSubcommand {
    Ferrisui(Cli),
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 📦 Scaffold a new FerrisUI desktop application project
    New {
        /// Project name or folder path
        #[arg(value_name = "PROJECT_NAME")]
        name: Option<String>,

        /// Starter template: "counter", "minimal", or "media"
        #[arg(short, long, value_name = "TEMPLATE")]
        template: Option<String>,
    },

    /// 🩺 Diagnose Rust toolchain, OS, and GPU/WGPU graphics adapter capabilities
    Doctor,

    /// ⚡ Run application with automatic file-watching and live recompilation
    Dev {
        /// Pass additional arguments directly to cargo run
        #[arg(last = true)]
        cargo_args: Vec<String>,
    },

    /// 🔨 Build and package application for release distribution
    Build {
        /// Target release build with optimizations
        #[arg(long, default_value_t = true)]
        release: bool,
    },
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = if args.len() > 1 && args[1] == "ferrisui" {
        match CargoCli::parse().command {
            CargoSubcommand::Ferrisui(cli) => cli.command,
        }
    } else {
        Cli::parse().command
    };

    let result = match command {
        Commands::New { name, template } => commands::new::execute(name, template),
        Commands::Doctor => commands::doctor::execute(),
        Commands::Dev { cargo_args } => commands::dev::execute(cargo_args),
        Commands::Build { release } => commands::build::execute(release),
    };

    if let Err(err) = result {
        eprintln!("\n{} {}", "❌ Error:".bold().red(), err);
        std::process::exit(1);
    }
}

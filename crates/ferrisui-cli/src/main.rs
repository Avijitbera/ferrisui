use clap::{Parser, Subcommand};

mod commands;
mod config;
mod templates;


#[derive(Parser, Debug)]
#[command(
    name= "ferrisui",
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

#[]
pub enum Commands {
    New {},
    Doctor,
    Dev {},
    Build {}
}



fn main() {
    println!("Hello, world!");
}

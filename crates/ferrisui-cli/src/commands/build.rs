use std::path::Path;
use std::process::Command;

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::AppManifest;

pub fn execute(release: bool) -> anyhow::Result<()> {
    println!(
        "{}",
        "🔨 Building FerrisUI Application Bundle\n".bold().cyan()
    );

    let manifest = AppManifest::load_from_dir(Path::new("."))?;

    let app_name = manifest.package.name;
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}")?);
    pb.set_message("Compiling optimized binaries with Cargo...");
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if release {
        cmd.arg("--release");
    }
    let status = cmd.status()?;
    if !status.success() {
        pb.finish_and_clear();
        anyhow::bail!("Compilation failed! See compiler output above.");
    }

    pb.finish_with_message("Compilation successfully!");
    let profile = if release { "release" } else { "debug" };
    let output_bin = Path::new("target").join(profile).join(&app_name);

    println!(
        "\n{} Successfully built application '{}'",
        "✔".green().bold(),
        app_name.bold()
    );

    if output_bin.exists() {
        if let Ok(metadata) = output_bin.metadata() {
            let size_mb = (metadata.len() as f64) / (1024.0 * 1024.0);
            println!(
                " Binary Location: {}",
                output_bin.display().to_string().cyan()
            );
            println!(" Binary Size: {:.2} MB", size_mb);
        }
    }
    Ok(())
}

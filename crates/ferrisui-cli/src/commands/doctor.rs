use colored::*;
use std::process::Command;

pub fn execute() -> anyhow::Result<()> {
    println!(
        "{}",
        "🩺 FerrisUI Environment & Hardware Diagnostics\n"
            .bold()
            .cyan()
    );
    // 1. Check OS and Architecture
    println!(
        "{} Operating System: {} ({})",
        "•".bold(),
        std::env::consts::OS.bold(),
        std::env::consts::ARCH
    );

    match Command::new("rustc").arg("--version").output() {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            println!("{} Rust Toolchain: {}", "✔".green().bold(), version);
        }
        _ => {
            println!(
                "{} Rust Toolchain: Not found! Please install Rust from https://rustup.rs",
                "✖".red().bold()
            );
        }
    }

    match Command::new("cargo").arg("--version").output() {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            println!("{} Cargo Manager:   {}", "✔".green().bold(), version);
        }
        _ => {
            println!("{} Cargo: Not found!", "✖".red().bold());
        }
    }

    println!("\n{}", "🎮 GPU & WGPU Rendering Acceleration:".bold());
    let instance = wgpu::Instance::default();
    let adapters = instance.enumerate_adapters(wgpu::Backends::all());
    if adapters.is_empty() {
        println!(
            " {} No compatible hardware GPU adapter discovered!",
            "⚠".yellow().bold()
        );
        println!("    Software fallback rendering will be utilized.");
    } else {
        for (idx, adapter) in adapters.iter().enumerate() {
            let info = adapter.get_info();
            println!(
                "  {} Adapter #{}: {} [{:?}] (Backend: {:?})",
                "✔".green().bold(),
                idx + 1,
                info.name.bold(),
                info.device_type,
                info.backend
            );
        }
    }

    println!(
        "\n{} Environment is ready for FerrisUI development.\n",
        "✔".green().bold()
    );
    Ok(())
}

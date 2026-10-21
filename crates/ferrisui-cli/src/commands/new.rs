use colored::*;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

use crate::{config::AppManifest, templates::TemplateKind};

pub fn execute(name_arg: Option<String>, template_arg: Option<String>) -> anyhow::Result<()> {
    println!("{}", "🦀 FerrisUI Project Generator\n".bold().cyan());
    let project_name = match name_arg {
        Some(name) => name,
        None => Input::<String>::new()
            .with_prompt("Enter projet name")
            .default("my_ferrisui_app".into())
            .interact_text()?,
    };
    let target_dir = Path::new(&project_name);
    if target_dir.exists() {
        anyhow::bail!("Directory {:?} already exists", project_name);
    }
    let template = match template_arg {
        Some(t_str) => TemplateKind::parse_str(&t_str).ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown template: {}, Options: counter, minimal, media",
                t_str
            )
        })?,
        None => {
            let options = [
                TemplateKind::Counter,
                TemplateKind::Minimal,
                TemplateKind::Media,
            ];
            let selection = Select::new()
                .with_prompt("Select a starter template")
                .default(0)
                .items(&options)
                .interact()?;
            options[selection]
        }
    };

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green}{msg}")?);
    pb.set_message("Scaffolding FerrisUI project structure...");

    fs::create_dir_all(target_dir.join("src"))?;
    fs::create_dir_all(target_dir.join("assets"))?;

    fs::write(
        target_dir.join("Cargo.toml"),
        template.generate_cargo_toml(&project_name),
    )?;

    fs::write(
        target_dir.join("src").join("main.rs"),
        template.generate_main_rs(&project_name),
    )?;

    fs::write(
        target_dir.join(".gitignore"),
        TemplateKind::generate_gitignore(),
    )?;

    let mut manifest = AppManifest::default();
    manifest.package.name = project_name.clone();
    manifest.window.title = project_name.clone();
    fs::write(target_dir.join("FerrisUI.toml"), manifest.to_toml_string()?)?;

    pb.finish_with_message("Done!");

    println!(
        "\n{} Successfully created project '{}' using the {:?} template!\n",
        "✅".green(),
        project_name,
        template
    );

    println!("To get started:\n");
    println!("{}", format!("  cd {}", project_name).bold());
    println!("   {}", "cargo run".bold());
    Ok(())
}

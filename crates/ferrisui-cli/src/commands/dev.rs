use colored::*;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::{Child, Command};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

pub fn execute(cargo_args: Vec<String>) -> anyhow::Result<()> {
    println!(
        "{} ",
        "⚡ Starting FerrisUI Live Dev Server...".bold().cyan()
    );

    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    let src_path = Path::new("src");
    if src_path.exists() {
        watcher.watch(src_path, RecursiveMode::Recursive)?;
        println!(
            "{} Watching '{}' for live changes",
            "•".bold(),
            "src/".cyan()
        );
    }

    let mut current_progress: Option<Child> = Some(spawn_app(&cargo_args)?);
    let mut last_reload = Instant::now();
    let debounce_duration = Duration::from_millis(500);

    println!("{} Press Ctrl+C to exit\n", "•".bold());

    for res in rx {
        match res {
            Ok(Event { kind, paths, .. }) if kind.is_modify() || kind.is_create() => {
                if last_reload.elapsed() < debounce_duration {
                    continue;
                }
                last_reload = Instant::now();

                let changed_file = paths
                    .first()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                println!(
                    "\n{} File changed ({}), rebuilding...",
                    "⚡".yellow().bold(),
                    changed_file.bold()
                );

                if let Some(mut child) = current_progress.take() {
                    let _ = child.kill();
                    let _ = child.wait();
                }

                current_progress = match spawn_app(&cargo_args) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        eprintln!("{} Failed to restart app: {}", "✗".red(), e);
                        None
                    }
                };
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
            _ => {}
        }
    }
    Ok(())
}

fn spawn_app(cargo_args: &[String]) -> anyhow::Result<Child> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run");
    for arg in cargo_args {
        cmd.arg(arg);
    }
    let child = cmd.spawn()?;
    Ok(child)
}

mod config;

use anyhow::{Context, Result};
use clap::Args;
use colour::*;
use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Instant;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

#[derive(Args)]
pub struct NewArgs {
    /// Name of the project
    pub name: Option<String>,
    /// Framework to use
    #[arg(short, long)]
    pub framework: Option<String>,
    /// Package manager to use
    #[arg(short = 'm', long)]
    pub package_manager: Option<String>,
    /// Plugins to include (comma-separated)
    #[arg(short, long)]
    pub plugins: Option<Vec<String>>,
}

fn run_command(command: &str, args: &[&str], pb: &ProgressBar) -> Result<()> {
    pb.set_message(format!("Running: {} {}", command, args.join(" ")));
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context(format!("Failed to execute command: {} {:?}", command, args))?;

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    stdout.lines()
        .chain(stderr.lines())
        .filter_map(Result::ok)
        .filter(|line| line.contains("error") || line.contains("warning") || line.contains("installed"))
        .for_each(|line| {
            if line.contains("error") {
                pb.suspend(|| red_ln!("{}", line));
            } else if line.contains("warning") {
                pb.suspend(|| yellow_ln!("{}", line));
            } else {
                pb.set_message(line);
            }
        });

    let status = child.wait()?;
    if !status.success() {
        pb.finish_with_message("Failed");
        red_ln!("Command failed: {} {:?}", command, args);
        anyhow::bail!("Command failed: {} {:?}", command, args);
    }
    pb.finish_with_message("Done");
    Ok(())
}

pub fn execute(args: &NewArgs) -> Result<()> {
    let started = Instant::now();
    let config = config::create_project_config(args)?;
    cyan_ln!("Creating new project: {}", config.name);
    blue_ln!("Framework: {} | Package Manager: {}", config.framework, config.package_manager);

    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")?
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    println!(
        "{} {}Initializing project...",
        style("[1/5]").bold().dim(),
        LOOKING_GLASS
    );

    // Initialize Nuxt project
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style.clone());
    run_command("npx", &["nuxi@latest", "init", "--packageManager", &config.package_manager, "--gitInit", &config.name], &pb)?;

    // Change to project directory
    std::env::set_current_dir(&config.name)?;

    println!(
        "{} {}Installing Nuxt modules...",
        style("[2/5]").bold().dim(),
        TRUCK
    );

    // Install Nuxt modules if applicable
    // Nuxt CLI doesn't support installing multiple modules at once
    let m = MultiProgress::new();
    let handles: Vec<_> = config.framework_modules.iter().enumerate().map(|(i, module)| {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(spinner_style.clone());
        pb.set_prefix(format!("[{}/{}]", i + 1, config.framework_modules.len()));
        let module = module.to_string();
        thread::spawn(move || {
            run_command("npx", &["nuxi@latest", "module", "add", &module], &pb).unwrap();
        })
    }).collect();

    for h in handles {
        let _ = h.join();
    }

    println!(
        "{} {}Adding Tauri dependencies...",
        style("[3/5]").bold().dim(),
        CLIP
    );

    // Add Tauri dependencies
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style.clone());
    run_command(&config.package_manager, &["add", "-D", "@tauri-apps/cli@next"], &pb)?;
    run_command(&config.package_manager, &["add", "@tauri-apps/api@next"], &pb)?;

    println!(
        "{} {}Initializing Tauri...",
        style("[4/5]").bold().dim(),
        PAPER
    );

    // Initialize Tauri
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style.clone());
    run_command(&config.package_manager, &[
        "tauri",
        "init",
        "--app-name", &config.name,
        "--window-title", &config.name,
        "--frontend-dist", "../dist",
        "--dev-url", "http://localhost:3000",
        "--before-dev-command", &format!("{} nuxt:dev", config.package_manager),
        "--before-build-command", &format!("{} generate", config.package_manager),
    ], &pb)?;

    println!(
        "{} {}Adding Tauri plugins...",
        style("[5/5]").bold().dim(),
        TRUCK
    );

    // Add Tauri plugins if applicable
    // Tauri CLI doesn't support adding multiple plugins at once yet
    let m = MultiProgress::new();
    let handles: Vec<_> = config.tauri_plugins.iter().enumerate().map(|(i, plugin)| {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(spinner_style.clone());
        pb.set_prefix(format!("[{}/{}]", i + 1, config.tauri_plugins.len()));
        let plugin = plugin.to_string();
        let package_manager = config.package_manager.clone();
        thread::spawn(move || {
            run_command(&package_manager, &["tauri", "add", &plugin], &pb).unwrap();
        })
    }).collect();

    for h in handles {
        let _ = h.join();
    }

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    green_ln!("Project {} created successfully!", config.name);

    Ok(())
}
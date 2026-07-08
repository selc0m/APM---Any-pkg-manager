use clap::Parser;
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser)]
#[command(name = "apm", version = "1.0", author = "selcom", about = "Advanced Package Manager - a universal wrapper")]
struct Cli {
    #[arg(short = 'S', long = "sync", conflicts_with_all = ["remove", "upgrade"], help = "Install a package")]
    sync: Option<String>,

    #[arg(short = 'R', long = "remove", conflicts_with_all = ["sync", "upgrade"], help = "Remove a package")]
    remove: Option<String>,

    #[arg(short = 'U', long = "upgrade", conflicts_with_all = ["sync", "remove"], help = "Upgrade system")]
    upgrade: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(pkg) = cli.sync {
        if confirm(&format!("Are you sure you want to install {}?", pkg)) {
            install_package(&pkg)?;
        }
    } else if let Some(pkg) = cli.remove {
        if confirm(&format!("Are you sure you want to remove {}?", pkg)) {
            remove_package(&pkg)?;
        }
    } else if cli.upgrade {
        if confirm("Are you sure you want to upgrade the system?") {
            upgrade_system()?;
        }
    } else {
        println!("Usage: apm [OPTIONS]");
        println!("Try 'apm --help' for more information.");
    }
    Ok(())
}

fn upgrade_system() -> anyhow::Result<()> {
    if command_exists("pacman") {
        let runner = if command_exists("yay") { "yay" } else { "sudo pacman" };
        run_cmd(runner, &["-Syu"]);
    } else if command_exists("apt") {
        run_cmd("sudo apt", &["update"]);
        run_cmd("sudo apt", &["upgrade", "-y"]);
    } else {
        eprintln!("Error: Package manager not found.");
    }
    Ok(())
}

fn confirm(prompt: &str) -> bool {
    print!("{} [Y/n]: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_lowercase();
    
    // Подтверждение при нажатии Enter (пустая строка) или вводе 'y'
    trimmed == "" || trimmed == "y"
}

fn install_package(package: &str) -> anyhow::Result<()> {
    if command_exists("pacman") {
        let runner = if command_exists("yay") { "yay" } else { "sudo pacman" };
        run_cmd(runner, &["-S", package]);
    } else if command_exists("apt") {
        run_cmd("sudo apt", &["install", package]);
    } else {
        eprintln!("Error: Package manager not found.");
    }
    Ok(())
}

fn remove_package(package: &str) -> anyhow::Result<()> {
    if command_exists("pacman") {
        run_cmd("sudo pacman", &["-Rns", package]);
    } else if command_exists("apt") {
        run_cmd("sudo apt", &["remove", package]);
    } else {
        eprintln!("Error: Package manager not found.");
    }
    Ok(())
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which").arg(cmd).output().is_ok()
}

fn run_cmd(runner: &str, args: &[&str]) {
    let parts: Vec<&str> = runner.split_whitespace().collect();
    let mut cmd = Command::new(parts[0]);
    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }
    cmd.args(args).status().expect("Error executing command");
}

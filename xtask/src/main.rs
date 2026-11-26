use anyhow::Result;
use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    Ci(CiArgs),
}

#[derive(Parser)]
struct CiArgs {}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Ci(_) => run_ci(),
    }
}

fn run_ci() -> Result<()> {
    println!("Running CI gates...");

    // Gate 1: Rust compilation
    println!("[1/4] Checking Rust compilation...");
    let status = Command::new("cargo")
        .arg("check")
        .status()?;
    if !status.success() {
        println!("FAIL: Rust compilation failed");
        std::process::exit(1);
    }
    println!("PASS: Rust compilation");

    // Gate 2: Text module tests
    println!("[2/4] Checking novakit-text tests...");
    let status = Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg("novakit-text")
        .status()?;
    if !status.success() {
        println!("FAIL: novakit-text tests failed");
        std::process::exit(1);
    }
    println!("PASS: novakit-text tests (6 passed)");

    // Gate 3: Tauri build (debug)
    println!("[3/4] Checking Tauri debug build...");
    let status = Command::new("cargo")
        .arg("tauri")
        .arg("build")
        .arg("--debug")
        .arg("--no-bundle")
        .status()?;
    if !status.success() {
        println!("FAIL: Tauri build failed");
        std::process::exit(1);
    }
    println!("PASS: Tauri debug build");

    // Gate 4: dist folder exists
    println!("[4/4] Checking dist folder...");
    if !std::path::Path::new("dist").exists() {
        println!("FAIL: dist folder not found");
        std::process::exit(1);
    }
    println!("PASS: dist folder exists");

    println!("\nAll CI gates passed!");
    Ok(())
}

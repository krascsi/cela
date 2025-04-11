use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use prettytable::{Table, row};
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Deserialize, Debug)]
struct CondaEnvironments {
    envs: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CondaPackage {
    name: String,
    version: String,
    channel: String,
}

#[derive(Subcommand)]
enum Commands {
    ListEnvs {},

    ListPackages {
        #[arg(required = true)]
        env_name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let result = match &cli.command {
        Commands::ListEnvs {} => list_environments(),
        Commands::ListPackages { env_name } => list_packages(env_name),
    };

    if let Err(err) = &result {
        eprintln!("Error: {}", err);

        if let Err(_) = Command::new("conda").arg("--version").output() {
            eprintln!("\nIt appears conda is not installed or in your PATH.");
            eprintln!("Please install conda or ensure it's properly configured.");
        }
    }

    result
}

fn list_environments() -> Result<()> {
    println!("Listing Conda environments...");

    let output = Command::new("conda")
        .args(["env", "list", "--json"])
        .output()
        .context("Failed to execute conda command. Is conda installed and in your PATH?")?;
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Conda command failed: {}", error));
    }
    let environments: CondaEnvironments =
        serde_json::from_slice(&output.stdout).context("Failed to parse JSON output from conda")?;

    println!("\nAvailable Conda environments:");
    println!("-------------------------------");

    for (i, path) in environments.envs.iter().enumerate() {
        let path_obj = Path::new(path);
        let env_name = path_obj
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(path);
        println!("{}. {} ({})", i + 1, env_name, path);
    }

    println!("\nTotal environments: {}", environments.envs.len());

    Ok(())
}

fn list_packages(env_name: &str) -> Result<()> {
    println!("Listing packages for environment: {}", env_name);

    let output = Command::new("conda")
        .args(["list", "-n", env_name, "--json"])
        .output()
        .context("Failed to execute conda command")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Conda command failed: {}", error));
    }

    let packages: Vec<CondaPackage> =
        serde_json::from_slice(&output.stdout).context("Failed to parse JSON output from conda")?;

    let mut table = Table::new();
    table.add_row(row!["Name", "Version", "Channel"]);

    for package in &packages {
        table.add_row(row![&package.name, &package.version, &package.channel]);
    }
    table.printstd();
    println!("\nTotal packages: {}", packages.len());

    Ok(())
}

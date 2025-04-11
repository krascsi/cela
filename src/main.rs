use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

    match &cli.command {
        Commands::ListEnvs {} => list_environments(),
        Commands::ListPackages { env_name } => list_packages(env_name),
    }
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
    let json: Value =
        serde_json::from_slice(&output.stdout).context("Failed to parse JSON output from conda")?;

    if let Some(envs) = json.get("envs") {
        if let Some(envs_array) = envs.as_array() {
            println!("\nAvailable Conda environments:");
            println!("-------------------------------");

            for (i, env) in envs_array.iter().enumerate() {
                if let Some(path) = env.as_str() {
                    let env_name = path.split('/').last().unwrap_or(path);
                    println!("{}. {} ({})", i + 1, env_name, path);
                }
            }

            println!("\nTotal environments: {}", envs_array.len());
        }
    }
    Ok(())
}

fn list_packages(env_name: &str) -> Result<()> {
    println!("Listing packages for environment: {}", env_name);
    Ok(())
}

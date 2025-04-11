use clap::{Parser, Subcommand};
use prettytable::{Table, row};
use serde::Deserialize;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

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

#[derive(Error, Debug)]
enum CondaError {
    #[error("Conda is not installer or not in your PATH")]
    NotInstalled,

    #[error("Conda command failed: {0}")]
    CommandFailed(String),

    #[error("Failed to parse Conda output: {0}")]
    ParseError(String),

    #[error("Conda environment '{0}' not found")]
    EnvironmentNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<serde_json::Error> for CondaError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParseError(err.to_string())
    }
}

type Result<T> = std::result::Result<T, CondaError>;

#[derive(Subcommand)]
enum Commands {
    ListEnvs {},

    ListPackages {
        #[arg(required = true)]
        env_name: String,
    },
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    check_conda_installed().map_err(|err| {
        eprintln!("Error: {}", err);
        eprintln!("Please install conda or ensure it's properly configured");
        err
    })?;

    let cli = Cli::parse();
    let result = match &cli.command {
        Commands::ListEnvs {} => list_environments(),
        Commands::ListPackages { env_name } => list_packages(env_name),
    };

    result.map_err(|err| {
        eprintln!("Error: {}", err);

        match &err {
            CondaError::EnvironmentNotFound(_) => {
                eprintln!("Tip: Run 'conda env list' to see available environment.")
            }
            CondaError::ParseError(_) => {
                eprintln!("This might be due to an unexpected format in conda's output.")
            }
            _ => {}
        }

        Box::new(err) as Box<dyn Error>
    })
}

fn run_conda_command<T: for<'de> Deserialize<'de>>(args: &[&str]) -> Result<T> {
    let output = Command::new("conda").args(args).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            return Err(CondaError::CommandFailed(stderr.to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            return Err(CondaError::CommandFailed(stdout.to_string()));
        }

        return Err(CondaError::CommandFailed(format!(
            "Command failed with exit code {}",
            output.status.code().unwrap_or(-1)
        )));
    }

    let result: T = serde_json::from_slice(&output.stdout)?;
    Ok(result)
}

fn check_conda_installed() -> Result<()> {
    Command::new("conda")
        .arg("--version")
        .output()
        .map(|_| ())
        .map_err(|_| CondaError::NotInstalled)
}

fn list_environments() -> Result<()> {
    println!("Listing Conda environments...");

    let environments: CondaEnvironments = run_conda_command(&["env", "list", "--json"])?;

    println!("\nAvailable Conda environments:");
    println!("-------------------------------");

    for (i, path) in environments.envs.iter().enumerate() {
        let index = i + 1;
        let path_obj = Path::new(path);
        let env_name = path_obj
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_else(|| path.split(std::path::MAIN_SEPARATOR).last().unwrap_or(path));
        println!("{index}. {env_name} ({path})")
        // println!("{}. {} ({})", i + 1, env_name, path);
    }

    println!("\nTotal environments: {}", environments.envs.len());

    Ok(())
}

fn list_packages(env_name: &str) -> Result<()> {
    println!("Listing packages for environment: {}", env_name);

    let packages = match run_conda_command::<Vec<CondaPackage>>(&["list", "-n", env_name, "--json"])
    {
        Ok(packages) => packages,
        Err(CondaError::CommandFailed(err)) if err.contains("not found") => {
            return Err(CondaError::EnvironmentNotFound(env_name.to_string()));
        }
        Err(err) => return Err(err),
    };

    let mut table = Table::new();
    table.add_row(row!["Name", "Version", "Channel"]);

    for package in &packages {
        table.add_row(row![&package.name, &package.version, &package.channel]);
    }
    table.printstd();
    println!("\nTotal packages: {}", packages.len());

    Ok(())
}

use std::{path::PathBuf, process::ExitStatus};

use acebau_database::Database;
use clap::{Parser, Subcommand};
use tokio::process::Command;
use url::Url;

#[derive(Debug, Parser)]
#[command(name = "acebau", about = "Acebau database administration")]
struct Cli {
    #[arg(long, env = "DATABASE_URL", hide_env_values = true)]
    database_url: Url,
    #[command(subcommand)]
    command: DatabaseCommand,
}

#[derive(Debug, Subcommand)]
enum DatabaseCommand {
    /// Create a PostgreSQL custom-format backup with pg_dump.
    Backup {
        #[arg(short, long)]
        output: PathBuf,
        /// Permit replacing an existing backup file.
        #[arg(long)]
        force: bool,
    },
    /// Permanently remove all application data from the public schema.
    Reset {
        /// Required acknowledgement for this destructive operation.
        #[arg(long)]
        confirm_reset: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        DatabaseCommand::Backup { output, force } => backup(&cli.database_url, output, force).await?,
        DatabaseCommand::Reset { confirm_reset } => reset(cli.database_url, confirm_reset).await?,
    }

    Ok(())
}

async fn backup(database_url: &Url, output: PathBuf, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    if output.exists() && !force {
        return Err(format!(
            "backup target {} already exists; pass --force to replace it",
            output.display()
        )
        .into());
    }

    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty()
        && !parent.is_dir()
    {
        return Err(format!("backup directory {} does not exist", parent.display()).into());
    }

    let status = Command::new("pg_dump")
        .arg("--format=custom")
        .arg("--file")
        .arg(&output)
        .env("PGDATABASE", database_url.as_str())
        .status()
        .await?;

    require_success(status, "pg_dump")?;
    println!("Backup created at {}", output.display());

    Ok(())
}

async fn reset(database_url: Url, confirm_reset: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !confirm_reset {
        return Err("database reset refused; pass --confirm-reset to acknowledge permanent data loss".into());
    }

    Database::connect(database_url).await?.reset_public_schema().await?;
    println!("Database public schema reset successfully");

    Ok(())
}

fn require_success(status: ExitStatus, command: &str) -> Result<(), Box<dyn std::error::Error>> {
    if status.success() {
        Ok(())
    } else {
        Err(format!("{command} exited with {status}").into())
    }
}

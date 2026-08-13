use std::{
    error::Error,
    path::{Path, PathBuf},
};

use acebau_database::{Database, MigrationOptions};
use clap::{ArgGroup, Args, Subcommand};
use clap_config_fallback::{ConfigArgs, ConfigSubcommand};
use snafu::Snafu;
use tokio::process::Command;
use url::Url;

#[derive(Debug, Snafu)]
enum BackupError {
    #[snafu(display("cannot dump database: {reason}"))]
    Dump { reason: String },
    #[snafu(display("cannot restore database: {reason}"))]
    Restore { reason: String },
}

#[derive(Debug, Subcommand, ConfigSubcommand)]
pub(crate) enum DatabaseCommand {
    /// Apply all application database migrations.
    Setup,
    /// Remove all tables and data, then set up the database again.
    Reset,
    /// Permanently remove all application data while preserving tables.
    Clear,
    /// Create or restore a PostgreSQL custom-format backup.
    Backup(BackupArgs),
}

#[derive(Debug, Args, ConfigArgs)]
#[group(skip)]
#[command(group(ArgGroup::new("mode").required(true).multiple(false)))]
pub(crate) struct BackupArgs {
    /// Dump the database to PATH.
    #[arg(long, value_name = "PATH", group = "mode")]
    dump: Option<PathBuf>,
    /// Restore the backup at PATH, replacing the current database contents.
    #[arg(long, value_name = "PATH", group = "mode")]
    restore: Option<PathBuf>,
    /// Permit replacing an existing dump file.
    #[arg(long, requires = "dump")]
    force: bool,
}

impl DatabaseCommand {
    pub(crate) async fn execute(self, database_url: Url) -> Result<(), Box<dyn Error>> {
        let database = Database::connect(database_url.clone()).await?;

        match self {
            Self::Setup => setup(&database).await,
            Self::Reset => reset(&database).await,
            Self::Clear => clear(&database).await,
            Self::Backup(args) => {
                if let Some(output) = args.dump {
                    dump(&database_url, &output, args.force).await
                } else if let Some(input) = args.restore {
                    restore(&database_url, &input).await
                } else {
                    unreachable!()
                }
            }
        }
    }
}

async fn setup(database: &Database) -> Result<(), Box<dyn Error>> {
    migrate(database).await?;
    println!("Database setup completed successfully");

    Ok(())
}

async fn migrate(database: &Database) -> Result<(), Box<dyn Error>> {
    database
        .migrate(
            &[
                &acebau_database::MIGRATOR,
                &acebau_machine::MIGRATOR,
                &acebau_activity::MIGRATOR,
                &acebau_catalogue::MIGRATOR,
                &acebau_part::MIGRATOR,
                &acebau_inventory::MIGRATOR,
                &acebau_recipe::MIGRATOR,
                &acebau_reseller::MIGRATOR,
                &acebau_order::MIGRATOR,
                &acebau_production::MIGRATOR,
                &acebau_invoice::MIGRATOR,
                &acebau_finance::MIGRATOR,
                &acebau_files::MIGRATOR,
                &acebau_settings::MIGRATOR,
            ],
            MigrationOptions::default(),
        )
        .await?;

    Ok(())
}

async fn dump(database_url: &Url, output: &Path, force: bool) -> Result<(), Box<dyn Error>> {
    if output.exists() && !force {
        return Err(BackupError::Dump {
            reason: format!("target {} already exists", output.display()),
        }
        .into());
    }

    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty()
        && !parent.is_dir()
    {
        return Err(BackupError::Dump {
            reason: format!("directory {} does not exist", parent.display()),
        }
        .into());
    }

    let status = Command::new("pg_dump")
        .arg("--format=custom")
        .arg("--no-owner")
        .arg("--no-privileges")
        .arg("--file")
        .arg(output)
        .env("PGDATABASE", database_url.as_str())
        .status()
        .await
        .map_err(|source| BackupError::Dump {
            reason: source.to_string(),
        })?;

    if !status.success() {
        return Err(BackupError::Dump {
            reason: format!("operation failed with {status}"),
        }
        .into());
    }

    println!("Backup created at {}", output.display());

    Ok(())
}

async fn restore(database_url: &Url, input: &Path) -> Result<(), Box<dyn Error>> {
    if !input.is_file() {
        return Err(BackupError::Restore {
            reason: format!("file {} does not exist", input.display()),
        }
        .into());
    }

    let status = Command::new("pg_restore")
        .arg("--clean")
        .arg("--if-exists")
        .arg("--no-owner")
        .arg("--no-privileges")
        .arg("--dbname")
        .arg(database_url.as_str())
        .arg(input)
        .status()
        .await
        .map_err(|source| BackupError::Restore {
            reason: source.to_string(),
        })?;

    if !status.success() {
        return Err(BackupError::Restore {
            reason: format!("operation failed with {status}"),
        }
        .into());
    }

    println!("Backup restored from {}", input.display());

    Ok(())
}

async fn reset(database: &Database) -> Result<(), Box<dyn Error>> {
    database.reset().await?;
    migrate(database).await?;
    println!("Database reset completed successfully");

    Ok(())
}

async fn clear(database: &Database) -> Result<(), Box<dyn Error>> {
    database.clear_all().await?;
    println!("Database data cleared successfully");

    Ok(())
}

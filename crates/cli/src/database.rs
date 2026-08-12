use std::path::PathBuf;

use acebau_database::{Database, MigrationOptions};
use clap::{ArgGroup, Args, Subcommand};
use clap_config_fallback::{ConfigArgs, ConfigSubcommand};
use url::Url;

#[derive(Debug, Subcommand, ConfigSubcommand)]
pub(crate) enum DatabaseCommand {
    /// Apply all application database migrations.
    Setup,
    /// Permanently remove all application data without removing tables.
    Reset {
        /// Required acknowledgement for this destructive operation.
        #[arg(long)]
        confirm: bool,
    },
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
    pub(crate) async fn execute(self, database_url: Url) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Setup => setup(database_url).await,
            Self::Reset { confirm } => reset(database_url, confirm).await,
            Self::Backup(args) => {
                if let Some(output) = args.dump {
                    dump(&database_url, output, args.force).await
                } else if let Some(input) = args.restore {
                    restore(&database_url, input).await
                } else {
                    unreachable!()
                }
            }
        }
    }
}

async fn setup(database_url: Url) -> Result<(), Box<dyn std::error::Error>> {
    Database::connect(database_url)
        .await?
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
    println!("Database setup completed successfully");

    Ok(())
}

async fn dump(database_url: &Url, output: PathBuf, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    Database::dump(database_url, output.clone(), force).await?;
    println!("Backup created at {}", output.display());

    Ok(())
}

async fn restore(database_url: &Url, input: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Database::restore(database_url, input.clone()).await?;
    println!("Backup restored from {}", input.display());

    Ok(())
}

async fn reset(database_url: Url, confirm_reset: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !confirm_reset {
        return Err("database reset refused; pass --confirm-reset to acknowledge permanent data loss".into());
    }

    Database::connect(database_url).await?.clear_all().await?;
    println!("Database data cleared successfully");

    Ok(())
}

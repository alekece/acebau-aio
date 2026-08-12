use std::{
    error::Error,
    path::{Path, PathBuf},
};

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
    pub(crate) async fn execute(self, database_url: Url) -> Result<(), Box<dyn Error>> {
        let database = Database::connect(database_url).await?;

        match self {
            Self::Setup => setup(&database).await,
            Self::Reset { confirm } => reset(&database, confirm).await,
            Self::Backup(args) => {
                if let Some(output) = args.dump {
                    dump(&database, &output, args.force).await
                } else if let Some(input) = args.restore {
                    restore(&database, &input).await
                } else {
                    unreachable!()
                }
            }
        }
    }
}

async fn setup(database: &Database) -> Result<(), Box<dyn Error>> {
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
    println!("Database setup completed successfully");

    Ok(())
}

async fn dump(database: &Database, output: &Path, force: bool) -> Result<(), Box<dyn Error>> {
    database.dump(output, force).await?;
    println!("Backup created at {}", output.display());

    Ok(())
}

async fn restore(database: &Database, input: &Path) -> Result<(), Box<dyn Error>> {
    database.restore(input).await?;
    println!("Backup restored from {}", input.display());

    Ok(())
}

async fn reset(database: &Database, confirm: bool) -> Result<(), Box<dyn Error>> {
    if !confirm {
        return Err("database reset refused".into());
    }

    database.clear_all().await?;
    println!("Database data cleared successfully");

    Ok(())
}

mod database;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_config_fallback::{ConfigParser, ConfigSubcommand};
use url::Url;

use crate::database::DatabaseCommand;

#[derive(Debug, Parser, ConfigParser)]
#[command(name = "acebau-cli", about = "Acebau administration")]
struct Cli {
    #[arg(long, env = "DATABASE_URL", hide_env_values = true)]
    database_url: Url,
    #[command(subcommand)]
    command: Command,
    #[arg(long, global = true)]
    #[config(path, format = "toml")]
    config_path: Option<PathBuf>,
}

#[derive(Debug, Subcommand, ConfigSubcommand)]
enum Command {
    /// Administer the PostgreSQL database.
    Database {
        #[command(subcommand)]
        command: DatabaseCommand,
    },
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse_with_config();

    match cli.command {
        Command::Database { command } => command.execute(cli.database_url).await,
    }
}

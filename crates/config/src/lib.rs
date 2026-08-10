#![allow(clippy::pedantic)]

use std::path::PathBuf;

use clap::Parser;
use clap_config_fallback::ConfigParser;
use url::Url;

#[derive(Debug, Parser, ConfigParser)]
pub struct ApiConfig {
    #[arg(short, long, env = "DATABASE_URL", hide_env_values = true)]
    pub database_url: Url,
    #[arg(long, env = "API_HOST")]
    pub host: String,
    #[arg(short, long, env = "API_PORT")]
    pub port: u16,
    #[arg(long)]
    #[config(path, format = "toml")]
    pub config_path: Option<PathBuf>,
}

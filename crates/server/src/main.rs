use std::path::PathBuf;

use acebau_database::Database;
use acebau_server::AppState;
use clap::Parser;
use clap_config_fallback::ConfigParser;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;

#[derive(Debug, Parser, ConfigParser)]
struct Cli {
    #[arg(short, long, env = "DATABASE_URL")]
    database_url: Url,
    #[arg(short, long, env = "SERVER_HOST")]
    host: String,
    #[arg(short, long, env = "SERVER_PORT")]
    port: u16,
    #[arg(long)]
    #[config(path, format = "toml")]
    config_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse_with_config();
    let database = Database::connect(cli.database_url).await?;
    let bind_address = (cli.host.as_str(), cli.port);

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(3600));
    let router = acebau_server::router(AppState::new(database)).layer(cors);
    let listener = tokio::net::TcpListener::bind(bind_address).await?;

    axum::serve(listener, router).await?;

    Ok(())
}

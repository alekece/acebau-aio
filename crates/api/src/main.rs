use acebau_api::AppState;
use acebau_config::ApiConfig;
use acebau_database::Database;
use clap_config_fallback::ConfigParser;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = ApiConfig::parse_with_config();
    let database = Database::connect(cli.database_url).await?;

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(3600));
    let router = acebau_api::router(AppState::new(database)).await?.layer(cors);
    let listener = tokio::net::TcpListener::bind((cli.host.as_str(), cli.port)).await?;

    axum::serve(listener, router).await?;

    Ok(())
}

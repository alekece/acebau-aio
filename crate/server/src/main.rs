use acebau_database::Database;
use acebau_server::AppState;
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, env = "DATABASE_URL")]
    database_url: Url,
    #[arg(short, long, env = "SERVER_PORT")]
    port: u16,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let database = Database::connect(args.database_url).await?;

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_method().allow_any_header().max_age(3600);

        App::new()
            .app_data(Data::new(AppState::new(database.clone())))
            .configure(acebau_server::registrer_routes)
            .wrap(cors)
    })
    .bind(("127.0.0.1", args.port))?
    .run()
    .await?;

    Ok(())
}

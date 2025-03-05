use actix_web::{App, HttpServer};
use eyre::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    #[cfg(feature = "color")]
    color_eyre::install()?;

    HttpServer::new(|| App::new().configure(acebau_backend::configure_app))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}

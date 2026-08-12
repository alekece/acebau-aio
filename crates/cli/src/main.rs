use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    acebau_cli::run().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    acebau_cli::run().await
}

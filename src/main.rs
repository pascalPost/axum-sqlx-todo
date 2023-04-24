use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rust_rest_api::run().await
}

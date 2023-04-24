use rust_rest_api::app;
use std::error::Error;

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = app().await?;
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

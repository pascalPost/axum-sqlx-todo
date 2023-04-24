use rust_rest_api::app;
use std::error::Error;
use std::net::TcpListener;

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap();
    tracing::debug!("listening on {}", &addr);

    println!("Webserver listening on {}", &addr);

    let app = app().await?;
    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

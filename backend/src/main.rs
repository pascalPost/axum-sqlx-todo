mod endpoints;
mod error;
mod state;

use axum::Router;
use dotenvy::dotenv;
use endpoints::{swagger_ui, todo};
use sqlx::SqlitePool;
use state::AppState;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load .env file if present
    // TODO remove for production use
    dotenv().ok();

    pretty_env_logger::init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    sqlx::migrate!().run(&pool).await?;

    let state = AppState::new(pool);

    let app = Router::new()
        .merge(swagger_ui::router())
        .merge(todo::router())
        // .layer(
        //     ServiceBuilder::new()
        //         .layer(HandleErrorLayer::new(|error: BoxError| async move {
        //             if error.is::<tower::timeout::error::Elapsed>() {
        //                 Ok(StatusCode::REQUEST_TIMEOUT)
        //             } else {
        //                 Err((
        //                     StatusCode::INTERNAL_SERVER_ERROR,
        //                     format!("Unhandled internal error: {}", error),
        //                 ))
        //             }
        //         }))
        //         .timeout(Duration::from_secs(10))
        //         .layer(TraceLayer::new_for_http())
        //         .into_inner(),
        // )
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

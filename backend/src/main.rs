mod endpoints;
mod error;
mod state;

use crate::endpoints::todo::{create_todo, list_todos, Todo, TodoEntry, TodoStatus};
use axum::{error_handling::HandleErrorLayer, routing, Router};
use dotenvy::dotenv;
use endpoints::todo;
use hyper::StatusCode;
use sqlx::{Pool, SqlitePool};
use state::AppState;
use std::env;
use std::error::Error;
use tower::{BoxError, ServiceBuilder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load .env file if present
    // TODO remove for production use
    dotenv().ok();

    pretty_env_logger::init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&pool).await?;

    let state = AppState::new(pool);

    // #[derive(OpenApi)]
    // #[openapi(
    //     paths(todo::list_todos, todo::create_todo),
    //     components(schemas(Todo, TodoEntry, TodoStatus)),
    //     tags((name = "todo", description = "Todo items management API"))
    // )]
    // struct ApiDoc;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup

    let app = todo::router()
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .route("/todo", routing::get(list_todos).post(create_todo))
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

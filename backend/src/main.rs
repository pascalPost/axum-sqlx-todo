mod todo;

use crate::todo::{create_todo, list_todos, Todo, TodoEntry, TodoStatus};
use axum::{routing, Router};
use dotenvy::dotenv;
use sqlx::SqlitePool;
use std::env;
use std::error::Error;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load .env file
    // TODO remove for production use
    dotenv().ok();

    pretty_env_logger::init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    #[derive(OpenApi)]
    #[openapi(
        paths(todo::list_todos, todo::create_todo),
        components(schemas(Todo, TodoEntry, TodoStatus)),
        tags((name = "todo", description = "Todo items management API"))
    )]
    struct ApiDoc;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/todo", routing::get(list_todos).post(create_todo))
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

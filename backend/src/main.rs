mod todo;

use crate::todo::{create_todo, list_todos, Todo, TodoEntry, TodoStatus};
use axum::{routing, Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // temporary todo in memory storage
    let storage = Mutex::new(Vec::<TodoEntry>::new());

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
        .with_state(Arc::new(storage));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
}

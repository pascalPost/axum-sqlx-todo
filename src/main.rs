use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{extract::State, routing, Json, Router};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

mod todo;
use crate::todo::Todo;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // temporary todo in memory storage
    let mut storage = Mutex::new(Vec::<Todo>::new());

    #[derive(OpenApi)]
    #[openapi(paths(list_todos), components(schemas(Todo)))]
    struct ApiDoc;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/todo", routing::get(list_todos))
        .with_state(Arc::new(storage))
        .route("/", routing::get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
}

/// list all todos
#[utoipa::path(
    get,
    path = "/todo",
    responses(
        (status = 200, description = "List all todos successfully", body = [Todo])
    )
)]
async fn list_todos(State(todos): State<Arc<Mutex<Vec<Todo>>>>) -> Json<Vec<Todo>> {
    let todos = todos.lock().await.clone();
    Json(todos)
}

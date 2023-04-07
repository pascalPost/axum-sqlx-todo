mod todo;

use crate::todo::{Todo, TodoEntry, TodoStatus};
use axum::{extract::State, response::IntoResponse, routing, Json, Router};
use hyper::StatusCode;
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
        paths(list_todos, create_todo),
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

/// list all todos
#[utoipa::path(
    get,
    path = "/todo",
    responses(
        (status = 200, description = "List all todos successfully", body = [TodoEntry])
    )
)]
async fn list_todos(State(todos): State<Arc<Mutex<Vec<TodoEntry>>>>) -> Json<Vec<TodoEntry>> {
    let todos = todos.lock().await.clone();
    Json(todos)
}

/// create a new todo
#[utoipa::path(
    post,
    path = "/todo",
    request_body = Todo,
    responses(
        (status = 201, description = "Todo item created successfully", body = TodoEntry),
    )
)]
async fn create_todo(
    State(todos): State<Arc<Mutex<Vec<TodoEntry>>>>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    let mut todos = todos.lock().await;
    let id = todos.len() as i32;
    let todo_entry = TodoEntry::new(id, todo);

    todos.push(todo_entry.clone());

    (StatusCode::CREATED, Json(todo_entry)).into_response()
}

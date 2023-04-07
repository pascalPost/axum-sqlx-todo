use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::ToSchema;

/// todo item
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Todo {
    #[schema(example = "Buy groceries")]
    value: String,
}

/// todo status enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub enum TodoStatus {
    Todo,
    InProgress,
    OnHold,
    Done,
}

/// todo database entry
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TodoEntry {
    id: i32,
    todo: Todo,
    status: TodoStatus,
}

impl TodoEntry {
    pub fn new(id: i32, todo: Todo) -> Self {
        Self {
            id,
            todo,
            status: TodoStatus::Todo,
        }
    }
}

/// list all todos
#[utoipa::path(
  get,
  path = "/todo",
  responses(
      (status = 200, description = "List all todos successfully", body = [TodoEntry])
  )
)]
pub async fn list_todos(State(todos): State<Arc<Mutex<Vec<TodoEntry>>>>) -> Json<Vec<TodoEntry>> {
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
pub async fn create_todo(
    State(todos): State<Arc<Mutex<Vec<TodoEntry>>>>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    let mut todos = todos.lock().await;
    let id = todos.len() as i32;
    let todo_entry = TodoEntry::new(id, todo);

    todos.push(todo_entry.clone());

    (StatusCode::CREATED, Json(todo_entry)).into_response()
}

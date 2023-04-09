use crate::error::AppError;
use crate::state::AppState;
use axum::{extract::State, response::IntoResponse, Json};
use axum::{routing, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(OpenApi)]
#[openapi(
    paths(list_todos, create_todo),
    components(schemas(Todo, TodoEntry, TodoStatus))
)]
pub struct ApiDocTodo;

/// returns the router for the todo endpoint
pub fn router() -> Router<AppState> {
    Router::new().route("/todo", routing::get(list_todos).post(create_todo))
}

/// todo item
#[derive(
    Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow, sqlx::Type, sqlx::Decode,
)]
pub struct Todo {
    #[schema(example = "Buy groceries")]
    value: String,
}

/// todo status enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Todo,
    InProgress,
    OnHold,
    Done,
}

/// todo database entry
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TodoEntry {
    id: i64,

    // has to be used here as sqlx compile time checked queries do not support
    // nested structs
    #[schema(example = "Buy groceries")]
    value: String,

    status: TodoStatus,
}

/// list all todos
#[utoipa::path(
  get,
  path = "/todo",
  responses(
      (status = 200, description = "List all todos successfully", body = [TodoEntry])
  )
)]
pub async fn list_todos(State(state): State<AppState>) -> Result<Json<Vec<TodoEntry>>, AppError> {
    let result = sqlx::query_as!(
        TodoEntry,
        r#"SELECT id, value, status as "status: TodoStatus" FROM todos"#,
    )
    .fetch_all(state.db_pool())
    .await?;
    Ok(Json(result))
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
    State(state): State<AppState>,
    Json(todo): Json<Todo>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.db_pool().acquire().await?;

    // Insert the todo, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
            INSERT INTO todos ( value )
            VALUES ( ?1 )
        "#,
        todo.value,
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok((StatusCode::CREATED, Json(id)).into_response())
}

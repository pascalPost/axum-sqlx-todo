use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use utoipa::ToSchema;

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

impl TodoEntry {
    pub fn new(id: i64, value: String) -> Self {
        Self {
            id,
            value,
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
pub async fn list_todos(
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<Vec<TodoEntry>>, StatusCode> {
    let result = sqlx::query_as!(
        TodoEntry,
        r#"SELECT id, value, status as "status: TodoStatus" FROM todos"#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    State(pool): State<Pool<Sqlite>>,
    Json(todo): Json<Todo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert the todo, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
            INSERT INTO todos ( value )
            VALUES ( ?1 )
        "#,
        todo.value,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .last_insert_rowid();

    Ok((StatusCode::CREATED, Json(id)).into_response())
}

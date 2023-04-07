use serde::{Deserialize, Serialize};
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

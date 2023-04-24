use sqlx::{Pool, Sqlite};

/// state used in this application
#[derive(Debug, Clone)]
pub struct AppState {
    db_pool: Pool<Sqlite>,
}

impl AppState {
    /// create a new state
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }

    /// returns the database pool
    pub fn db_pool(&self) -> &Pool<Sqlite> {
        &self.db_pool
    }
}

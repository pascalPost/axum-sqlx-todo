use crate::state::AppState;
use axum::Router;

pub mod health_check;
pub mod swagger_ui;
pub mod todo;

/// returns the router to all endpoint
pub fn router() -> Router<AppState> {
    Router::new()
        .merge(swagger_ui::router())
        .merge(health_check::router())
        .merge(todo::router())
}

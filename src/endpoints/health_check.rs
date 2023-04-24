use crate::{error::AppError, state::AppState};
use axum::{routing, Router};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(health_check))]
pub struct ApiDoc;

/// returns the router for the todo endpoint
pub fn router() -> Router<AppState> {
    Router::new().route("/health_check", routing::get(health_check))
}

/// health check endpoint
#[utoipa::path(
  get,
  path = "/health_check",
  responses(
      (status = 200, description = "Application up and running")
  )
)]
pub async fn health_check() -> Result<(), AppError> {
    Ok(())
}

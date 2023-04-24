use crate::state::AppState;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::todo::ApiDocTodo;

#[derive(OpenApi)]
#[openapi(
    tags((name = "todo", description = "Todo items management API"))
)]
struct ApiDoc;

fn openapi() -> utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();
    doc.merge(ApiDocTodo::openapi());
    doc
}

/// returns the router for the swagger-ui endpoint
pub fn router() -> Router<AppState> {
    Router::new().merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi()))
}

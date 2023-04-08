mod todo;

use crate::todo::{create_todo, list_todos, Todo, TodoEntry, TodoStatus};
use axum::{extract::MatchedPath, routing, Router};
use dotenvy::dotenv;
use hyper::Request;
use sqlx::SqlitePool;
use std::env;
use std::error::Error;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load .env file
    // TODO remove for production use
    dotenv().ok();

    // // temporary todo in memory storage
    // let storage = Mutex::new(Vec::<TodoEntry>::new());

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    #[derive(OpenApi)]
    #[openapi(
        paths(todo::list_todos, todo::create_todo),
        components(schemas(Todo, TodoEntry, TodoStatus)),
        tags((name = "todo", description = "Todo items management API"))
    )]
    struct ApiDoc;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/todo", routing::get(list_todos).post(create_todo))
        .with_state(pool)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

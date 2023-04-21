#![allow(unused)]

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Error handling for the API.
/// Taken from https://github.com/launchbadge/realworld-axum-sqlx/blob/main/src/http/error.rs

/// A common error type that can be used throughout the API.
///
/// Can be returned in a `Result` from an API handler function.
///
/// For convenience, this represents both API errors as well as internal recoverable errors,
/// and maps them to appropriate status codes along with at least a minimally useful error
/// message in a plain text body, or a JSON body in the case of `UnprocessableEntity`.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    NotFound,

    /// Automatically return `500 Internal Server Error` on a `sqlx::Error`.
    ///
    /// Via the generated `From<sqlx::Error> for Error` impl,
    /// this allows using `?` on database calls in handler functions without a manual mapping step.
    /// Otherwise `.map_err(ErrInternalServerError)?` would be needed.
    ///
    /// The actual error message isn't returned to the client for security reasons.
    /// It should be logged instead.
    ///
    /// Note that this could also contain database constraint errors, which should usually
    /// be transformed into client errors (e.g. `422 Unprocessable Entity` or `409 Conflict`).
    /// See `ResultExt` below for a convenient way to do this.
    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),
}

impl AppError {
    /// Return the HTTP status code that should be used for this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Implementation of IntoResponse for AppError. This function is called when an
/// AppError is returned from a handler function.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            AppError::Unauthorized => (),
            AppError::Forbidden => (),
            AppError::NotFound => (),
            AppError::Sqlx(e) => {
                tracing::error!("SQLx error: {:?}", e);
            }
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

//! Utilities used in multiple routes

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

/// See <https://docs.rs/axum/latest/axum/error_handling/index.html>
pub struct AppError(color_eyre::eyre::Error);
// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<color_eyre::eyre::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

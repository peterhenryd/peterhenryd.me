use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct AppError(anyhow::Error);

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {}", self.0),
        ).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(value: E) -> Self {
        Self(value.into())
    }
}
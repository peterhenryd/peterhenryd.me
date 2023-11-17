use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::warn;

pub type AppResult<T> = Result<T, AppError>;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = format!("internal server error: {}", self.0);

        warn!("{}", &error_message);

        (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

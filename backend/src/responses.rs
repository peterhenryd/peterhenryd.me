use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub trait IntoDescriptiveResponse {
    fn into_descriptive_response(self) -> Response;
}

#[derive(Serialize)]
struct DescriptiveResponse {
    message: &'static str,
}

impl IntoDescriptiveResponse for StatusCode {
    fn into_descriptive_response(self) -> Response {
        let message = self.canonical_reason().unwrap();
        let descriptive_response = DescriptiveResponse { message };
        let json = Json(descriptive_response);

        (self, json).into_response()
    }
}

#[derive(Serialize)]
struct ValueResponse<T: Serialize> {
    value: T
}

pub trait IntoValueResponse {
    fn into_value_response(self) -> Response;
}

impl<T: Serialize> IntoValueResponse for T {
    fn into_value_response(self) -> Response {
        let response = ValueResponse { value: self };

        Json(response).into_response()
    }
}
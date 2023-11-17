use axum::response::{IntoResponse, Redirect, Response};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiQuery {
    redirect: Option<String>,
}

impl ApiQuery {
    pub fn with_default(self, default: impl IntoResponse) -> Response {
        match self.redirect {
            None => default.into_response(),
            Some(s) => Redirect::to(&s).into_response(),
        }
    }
}

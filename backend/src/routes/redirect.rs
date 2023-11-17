use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use crate::error::AppResult;

#[derive(Deserialize, Debug)]
pub struct Redirect {
    redirect: Option<String>
}

impl Redirect {
    pub fn response<R: IntoResponse>(self, other: impl FnOnce() -> AppResult<R>) -> AppResult<Response> {
        let other = other()?;

        match self.redirect {
            Some(string) => Ok(axum::response::Redirect::to(string.as_str()).into_response()),
            None => Ok(other.into_response())
        }
    }
}
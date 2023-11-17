use axum::extract::{Query, State};
use axum::Form;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use tower_sessions::Session;
use crate::api_query::ApiQuery;
use crate::app_error::AppResult;
use crate::app_state::AppState;
use crate::responses::IntoDescriptiveResponse;

pub async fn fetch_session_handler(
    session: Session,
    Query(query): Query<ApiQuery>
) -> AppResult<Response> {
    let is_admin = session.get::<()>("admin")?.is_some();
    let response = is_admin.to_string().into_response();

    Ok(query.with_default(response))
}

#[derive(Deserialize)]
pub struct CreateSessionForm {
    admin_key: String
}

pub async fn create_session_handler(
    State(AppState { admin_key, .. }): State<AppState>,
    session: Session,
    Query(query): Query<ApiQuery>,
    Form(create_session): Form<CreateSessionForm>,
) -> AppResult<Response> {
    let is_admin = admin_key == create_session.admin_key;

    if is_admin {
        session.insert("admin", ())?;
    }

    Ok(query.with_default(is_admin.to_string()))
}

pub async fn delete_session_handler(
    session: Session,
    Query(query): Query<ApiQuery>
) -> AppResult<Response> {
    let status_code =
        if session.remove::<()>("admin")?.is_some() {
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        };
    let response = status_code.into_descriptive_response();

    Ok(query.with_default(response))
}
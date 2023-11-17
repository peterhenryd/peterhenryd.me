use axum::extract::{Query, State};
use axum::{Form, Json};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use tower_sessions::Session;
use crate::error::AppResult;
use crate::routes::redirect::Redirect;
use crate::state::AppState;

pub async fn fetch_session_handler(session: Session, Query(redirect): Query<Redirect>) -> AppResult<Response> {
    dbg!(&redirect);

    redirect.response(move || Ok(Json(session.get::<()>("admin")?.is_some())))
}

#[derive(Deserialize)]
pub struct CreateSession {
    admin_key: String
}

pub async fn create_session_handler(
    State(AppState { admin_key, .. }): State<AppState>,
    session: Session,
    Query(redirect): Query<Redirect>,
    Form(create_session): Form<CreateSession>,
) -> AppResult<Response> {
    redirect.response(move ||
        Ok(Json(if admin_key == create_session.admin_key {
            session.insert("admin", ())?;
            true
        } else {
            false
        }))
    )
}

pub async fn delete_session_handler(session: Session, Query(redirect): Query<Redirect>) -> AppResult<Response> {
    redirect.response(||
        Ok(if session.remove::<()>("admin")?.is_some() {
            "DELETED"
        } else {
            "NOTHING_TO_DELETE"
        })
    )
}
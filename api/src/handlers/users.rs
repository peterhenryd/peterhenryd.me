use axum::extract::State;
use axum::{Form, Json};
use tower_sessions::Session;
use serde::{Deserialize, Serialize};
use peterhenryd_me_lib::ConnectionPool;
use crate::error::AppResult;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct UserSession {
    id: i64,
}

pub async fn fetch_session_handler(session: Session) -> AppResult<Json<Option<UserSession>>> {
    Ok(Json(session.get::<UserSession>("user_session")?))
}

#[derive(Deserialize)]
pub struct CreateSession {
    username: String,
    password: String,
}

pub async fn create_session_handler(
    State(AppState { pool }): State<AppState>,
    session: Session,
    Form(create_session): Form<CreateSession>
) -> AppResult<Json<UserSession>> {

}

pub async fn delete_session_handler(State(AppState { pool }): State<AppState>) ->  {

}

pub async fn create_handler(State(AppState { pool }): State<AppState>) ->  {

}

pub async fn load_all_handler(State(AppState { pool }): State<AppState>) ->  {

}

pub async fn load_by_id_handler(State(AppState { pool }): State<AppState>) ->  {

}
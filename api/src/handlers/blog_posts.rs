use axum::extract::{Path, State};
use axum::Json;
use peterhenryd_me_lib::models::blog_posts::{BlogPost, load_all, load_by_id};
use crate::error::AppResult;
use crate::state::AppState;

pub async fn load_all_handler(State(AppState { pool }): State<AppState>) -> AppResult<Json<Vec<BlogPost>>> {
    let blog_posts = load_all(&mut pool.get()?)?;
    Ok(Json(blog_posts))
}

pub async fn load_by_id_handler(State(AppState { pool }): State<AppState>, Path(id): Path<i64>) -> AppResult<Json<Option<BlogPost>>> {
    let blog_post = load_by_id(&mut pool.get()?, id)?;
    Ok(Json(blog_post))
}
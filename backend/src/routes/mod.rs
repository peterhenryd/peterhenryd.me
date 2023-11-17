use axum::Router;
use axum::routing::{delete, get, post};
use crate::app_state::AppState;

mod blog_post;
mod admin;

/// API Description:
///   GET  /blog-posts/all
///   GET  /blog-posts/id/:id
///   POST /blog-posts/new
///   GET  /admin
///   POST /admin
pub fn router(state: AppState) -> Router<()> {
    Router::new()
        .route("/blog-posts/all", get(blog_post::fetch_all_handler))
        .route("/blog-posts/id/:id", get(blog_post::fetch_by_id_handler))
        .route("/blog-posts/new", post(blog_post::create_handler))
        .route("/admin", get(admin::fetch_session_handler))
        .route("/admin", post(admin::create_session_handler))
        .route("/admin", delete(admin::delete_session_handler))
        .with_state(state)
}
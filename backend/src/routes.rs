use axum::Router;
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
        .with_state(state)
        .nest("/blog-posts", blog_post::router())
        .nest("/admin", admin::router())
}
use crate::app_state::AppState;
use axum::Router;

mod admin;
mod blog_post;

/// API Description:
///   GET  /blog-posts/all
///   GET  /blog-posts/id/:id
///   POST /blog-posts/new
///   GET  /admin
///   POST /admin
pub fn router(state: AppState) -> Router<()> {
    Router::new()
        .nest("/blog-posts", blog_post::router())
        .nest("/admin", admin::router())
        .with_state(state)
}

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Form, Json};
use axum::response::Response;
use diesel::{ExpressionMethods, insert_into, Insertable, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use time::OffsetDateTime;
use crate::app_error::AppResult;
use crate::models::blog_post::BlogPost;
use crate::app_state::AppState;
use crate::responses::{IntoDescriptiveResponse, IntoValueResponse};

pub async fn fetch_all_handler(
    State(AppState { pool, .. }): State<AppState>
) -> AppResult<Json<Vec<BlogPost>>> {
    use crate::schema::blog_posts::dsl;

    Ok(Json(dsl::blog_posts.load::<BlogPost>(&mut pool.get()?)?))
}

pub async fn fetch_by_id_handler(
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<i64>
) -> AppResult<Result<Json<BlogPost>, Response>> {
    use crate::schema::blog_posts::dsl;

    let blog_posts = dsl::blog_posts
        .filter(dsl::id.eq(id))
        .load::<BlogPost>(&mut pool.get()?)?;
    let blog_post = blog_posts
        .into_iter()
        .next();
    let err = StatusCode::NOT_FOUND.into_descriptive_response();
    let response = blog_post
        .map(Json::from)
        .ok_or(err);

    Ok(response)
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::blog_posts)]
pub struct CreationForm {
    pub title: String,
    pub tagline: String,
    pub content: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(default)]
    pub published_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub edited_at: Option<OffsetDateTime>,
}

pub async fn create_handler(
    State(AppState { pool, .. }): State<AppState>,
    Form(create_blog_post): Form<CreationForm>
) -> AppResult<Response> {
    use crate::schema::blog_posts::dsl;

    let id = insert_into(dsl::blog_posts)
        .values(create_blog_post)
        .returning(dsl::id)
        .get_result::<i64>(&mut pool.get()?)?;

    Ok(id.into_value_response())
}
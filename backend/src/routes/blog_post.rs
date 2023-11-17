use axum::Router;
use axum::routing::get;
use diesel::{ExpressionMethods, Insertable, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use crate::app_state::AppState;
use crate::responses::{IntoDescriptiveResponse, IntoValueResponse};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/all", get(handlers::all::get))
        .route("/id/:id", get(handlers::id::get))
        .route("/", get(handlers::post))
}

mod handlers {
    use axum::extract::State;
    use axum::Form;
    use axum::response::Response;
    use diesel::{insert_into, Insertable, RunQueryDsl};
    use serde::Deserialize;
    use time::OffsetDateTime;
    use crate::app_error::AppResult;
    use crate::app_state::AppState;
    use crate::responses::IntoValueResponse;

    pub mod all {
        use axum::Json;
        use crate::models::blog_post::BlogPost;
        use super::*;

        pub async fn get(
            State(AppState { pool, .. }): State<AppState>
        ) -> AppResult<Json<Vec<BlogPost>>> {
            use crate::schema::blog_posts::dsl;

            Ok(Json(dsl::blog_posts.load::<BlogPost>(&mut pool.get()?)?))
        }
    }

    pub mod id {
        use axum::extract::Path;
        use axum::http::StatusCode;
        use axum::Json;
        use diesel::ExpressionMethods;
        use crate::models::blog_post::BlogPost;
        use crate::responses::IntoDescriptiveResponse;
        use super::*;

        pub async fn get(
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

    pub async fn post(
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
}
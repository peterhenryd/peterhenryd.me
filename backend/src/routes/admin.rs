use axum::Router;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use serde::Deserialize;
use crate::app_state::AppState;
use crate::responses::IntoDescriptiveResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::get))
        .route("/", post(handlers::post))
        .route("/", delete(handlers::delete))
}

mod handlers {
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

    pub async fn get(
        session: Session,
        Query(query): Query<ApiQuery>,
    ) -> AppResult<Response> {
        let is_admin = session.get::<()>("admin")?.is_some();
        let response = is_admin.to_string().into_response();

        Ok(query.with_default(response))
    }

    #[derive(Deserialize)]
    pub struct LoginForm {
        admin_key: String
    }

    pub async fn post(
        State(AppState { admin_key, .. }): State<AppState>,
        session: Session,
        Query(query): Query<ApiQuery>,
        Form(login): Form<LoginForm>,
    ) -> AppResult<Response> {
        let is_admin = admin_key == login.admin_key;

        if is_admin {
            session.insert("admin", ())?;
        }

        Ok(query.with_default(is_admin.to_string()))
    }

    pub async fn delete(
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
}
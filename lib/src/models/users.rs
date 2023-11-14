use diesel::{ExpressionMethods, Queryable, QueryDsl, QueryResult, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::{Connection, ConnectionPool};
use crate::schema::blog_posts::dsl;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub prisms_id: i64,
    pub created_at: OffsetDateTime,
    pub last_logged_in_at: OffsetDateTime,
    pub is_admin: bool,
}

pub fn load_by_username(pool: &ConnectionPool, username: &str) {
    use crate::schema::users::dsl;

    dsl::users.filter(dsl::)
}
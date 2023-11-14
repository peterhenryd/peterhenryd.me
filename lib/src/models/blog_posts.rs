use diesel::{ExpressionMethods, Queryable, QueryDsl, QueryResult, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::Connection;
use crate::schema::blog_posts::dsl;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::blog_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub tagline: String,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub published_at: Option<OffsetDateTime>,
    pub edited_at: Option<OffsetDateTime>,
    pub author_id: i64,
}

pub fn load_all(conn: &mut Connection) -> QueryResult<Vec<BlogPost>> {
    dsl::blog_posts
        .load::<BlogPost>(conn)
}

pub fn load_by_id(conn: &mut Connection, id: i64) -> QueryResult<Option<BlogPost>> {
    dsl::blog_posts
        .filter(dsl::id.eq(id))
        .load::<BlogPost>(conn)
        .map(|vec| vec.into_iter().next())
}
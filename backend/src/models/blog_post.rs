use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize, Serializer};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::blog_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub tagline: String,
    pub content: String,
    #[serde(serialize_with = "time::serde::iso8601::serialize")]
    pub created_at: OffsetDateTime,
    #[serde(serialize_with = "iso8601_option")]
    pub published_at: Option<OffsetDateTime>,
    #[serde(serialize_with = "iso8601_option")]
    pub edited_at: Option<OffsetDateTime>,
}

pub fn iso8601_option<S: Serializer>(
    datetime: &Option<OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match datetime {
        Some(date) => time::serde::iso8601::serialize(date, serializer),
        x => x.serialize(serializer),
    }
}

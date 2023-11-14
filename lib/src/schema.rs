// @generated automatically by Diesel CLI.

diesel::table! {
    blog_post_comments (id) {
        id -> Int8,
        blog_post_id -> Int8,
        author_id -> Int8,
        content -> Text,
        created_at -> Timestamptz,
        edited_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    blog_posts (id) {
        id -> Int8,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        tagline -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        published_at -> Nullable<Timestamptz>,
        edited_at -> Nullable<Timestamptz>,
        author_id -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        prisms_id -> Int8,
        created_at -> Timestamptz,
        last_logged_in_at -> Timestamptz,
        is_admin -> Bool,
    }
}

diesel::joinable!(blog_post_comments -> blog_posts (blog_post_id));
diesel::joinable!(blog_post_comments -> users (author_id));
diesel::joinable!(blog_posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog_post_comments,
    blog_posts,
    users,
);

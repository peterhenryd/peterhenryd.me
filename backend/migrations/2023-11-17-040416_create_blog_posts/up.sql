create table blog_posts
(
    id           bigserial
        primary key,
    title        varchar(255)             not null,
    tagline      varchar(255)             not null,
    content      text                     not null,
    created_at   timestamp with time zone not null,
    published_at timestamp with time zone,
    edited_at    timestamp with time zone
);
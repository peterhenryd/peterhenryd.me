use yew::prelude::*;
use askama::Template;
use crate::layouts::default::DefaultLayout;
use crate::html::ToHtml;
#[derive(Template)]
#[template(path = "pages/blog_post.html")]
struct BlogPostPage;

pub fn render() -> Html {
    html! {
        <DefaultLayout>
            { BlogPostPage.into_html() }
        </DefaultLayout>
    }
}
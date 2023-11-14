use yew::prelude::*;
use askama::Template;
use crate::html::ToHtml;
use crate::layouts::default::DefaultLayout;

#[derive(Template)]
#[template(path = "pages/blog.html")]
pub struct BlogPage;

pub fn render() -> Html {
    html! {
        <DefaultLayout>
            { BlogPage.into_html() }
        </DefaultLayout>
    }
}
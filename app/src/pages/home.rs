use askama::Template;
use yew::{Html, html};
use crate::html::ToHtml;
use crate::layouts::default::DefaultLayout;

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomePage;

pub fn render() -> Html {
    html! {
        <DefaultLayout>
            { HomePage.into_html() }
        </DefaultLayout>
    }
}
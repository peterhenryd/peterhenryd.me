use askama::Template;
use yew::Html;
use crate::html::ToHtml;

#[derive(Template)]
#[template(path = "pages/not_found.html")]
struct NotFound;

pub fn render() -> Html {
    NotFound.into_html()
}
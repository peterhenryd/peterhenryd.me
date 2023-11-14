use askama::Template;
use yew::{AttrValue, Html};

pub trait ToHtml: Sized {
    fn to_html(&self) -> Html;

    fn into_html(self) -> Html {
        self.to_html()
    }
}

impl<T: Template> ToHtml for T {
    fn to_html(&self) -> Html {
        let html = self.render().unwrap();
        let attr_value = AttrValue::from(html);
        Html::from_html_unchecked(attr_value)
    }
}
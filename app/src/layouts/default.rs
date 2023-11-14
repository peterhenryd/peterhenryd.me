use yew::prelude::*;
use crate::components::footer::Footer;
use crate::components::header::Header;

#[derive(Properties, PartialEq)]
pub struct DefaultLayoutProps {
    pub children: Html
}

#[function_component(DefaultLayout)]
pub fn default_layout(props: &DefaultLayoutProps) -> Html {
    html! {
        <div class="flex flex-col gap-10" style="margin: 50px 20%;">
            <Header />
            { props.children.clone() }
            <Footer />
        </div>
    }
}
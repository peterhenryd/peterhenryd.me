use yew::prelude::*;

pub mod link;
pub mod route_link;

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub children: Children
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    html! {
        <nav class="nav flex flex-row gap-4">
            { props.children.clone() }
        </nav>
    }
}
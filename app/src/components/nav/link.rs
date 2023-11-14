use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub children: Children,
    pub href: String,
    #[prop_or_default]
    pub target: String,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    let href = props.href.clone();
    let target = props.target.clone();

    let mut classes = props.classes.clone();
    classes.extend(classes!("nav-link", "text-xl"));

    html! {
        <a class={classes} {href} {target}>
            { props.children.clone() }
        </a>
    }
}
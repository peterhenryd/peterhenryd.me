use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct NavRouteLinkProps {
    pub children: Html,
    pub to: Route,
}

#[function_component(NavRouteLink)]
pub fn nav_route_link(props: &NavRouteLinkProps) -> Html {
    let route = use_route::<Route>().unwrap();

    let mut classes = classes!("nav-link", "text-xl");
    if route == props.to {
        classes.push("nav-link-active");
    }

    html! {
        <Link<Route> to={props.to} {classes}>
            { props.children.clone() }
        </Link<Route>>
    }
}
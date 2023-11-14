use yew::prelude::*;
use crate::components::nav::Nav;
use crate::components::nav::route_link::NavRouteLink;
use crate::components::theme_toggler::ThemeToggler;
use crate::router::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <Nav>
            <NavRouteLink to={Route::Home}>{"Home."}</NavRouteLink>
            <NavRouteLink to={Route::Blog}>{"Blog."}</NavRouteLink>
            <ThemeToggler />
        </Nav>
    }
}
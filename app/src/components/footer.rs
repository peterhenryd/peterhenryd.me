use yew::prelude::*;
use crate::components::nav::link::NavLink;
use crate::components::nav::Nav;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <Nav>
            <NavLink target="_blank" href="https://github.com/peterhenryd">
                {"GitHub."}
            </NavLink>
            <NavLink target="_blank" href="https://linkedin.com/in/peterhenryd">
                {"LinkedIn."}
            </NavLink>
            <NavLink target="_blank" href="https://github.com/peterhenryd/peterhenryd-me" classes={classes!("ml-auto")}>
                {"Source Code."}
            </NavLink>
        </Nav>
    }
}
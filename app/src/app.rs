use bounce::BounceRoot;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::theme::Theme;
use crate::title::TitleApplier;

#[function_component(App)]
pub fn app() -> Html {
    Theme::init();

    html! {
        <BrowserRouter>
            <BounceRoot>
                <TitleApplier />
                <Switch<Route> render={Route::switch} />
            </BounceRoot>
        </BrowserRouter>
    }
}
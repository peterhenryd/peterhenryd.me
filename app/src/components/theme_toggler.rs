use yew::prelude::*;
use crate::theme::Theme;

#[function_component(ThemeToggler)]
pub fn theme_toggler() -> Html {
    let theme = use_state(Theme::get);
    let onclick = {
        let theme = theme.clone();
        Callback::from(move |_| {
            theme.set(theme.toggle());
        })
    };

    html! {
        <button {onclick}>
            { theme.icon() }
        </button>
    }
}
use std::rc::Rc;
use bounce::use_artifacts;
use gloo::utils::document;
use yew::prelude::*;

#[derive(PartialEq, Eq)]
pub struct Title {
    value: String,
}

impl Title {
    pub fn new_rc(value: &'static str) -> Rc<Self> {
        Rc::new(Self { value: value.into() })
    }
}

#[function_component(TitleApplier)]
pub fn title_applier() -> Html {
    let titles = use_artifacts::<Title>();

    let title = titles
        .last()
        .map(|title| title.value.to_owned())
        .map(|title| format!("{} - peterhenryd.me", title))
        .unwrap_or_else(|| "peterhenryd.me".into());

    use_effect_with(title, |title| {
        document().set_title(title);

        || {}
    });

    Html::default()
}
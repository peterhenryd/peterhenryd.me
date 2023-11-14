use std::str::FromStr;
use gloo::utils::document;
use gloo_storage::{LocalStorage, Storage};
use yew::{Html, html};

#[derive(Copy, Clone)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn get() -> Theme {
        LocalStorage::raw()
            .get("theme")
            .unwrap()
            .map(|s| Theme::from_str(s.as_str()))
            .map(Result::unwrap)
            .unwrap_or(Theme::Light)
    }

    pub fn init() {
        Theme::set(Self::get());
    }

    pub fn set(theme: Theme) {
        let storage = LocalStorage::raw();

        document()
            .get_element_by_id("html")
            .unwrap()
            .set_attribute("data-theme", theme.as_str())
            .unwrap();

        storage.set("theme", theme.as_str()).unwrap();
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn toggle(self) -> Theme {
        let theme = match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };

        Theme::set(theme);

        theme
    }

    pub fn icon(self) -> Html {
        let class = match self {
            Theme::Light => "fa-solid fa-sun",
            Theme::Dark => "fa-solid fa-moon",
        };

        html! { <i {class} style="font-size: auto;"></i> }
    }
}

impl FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err(())
        }
    }
}
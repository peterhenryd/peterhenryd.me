use crate::app::App;

mod app;
mod router;
mod html;
mod pages;
mod title;
mod components;
mod layouts;
mod theme;

fn main() {
    yew::Renderer::<App>::new().render();
}

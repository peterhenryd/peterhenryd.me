use bounce::Artifact;
use yew::prelude::*;
use yew_router::Routable;
use crate::pages;
use crate::title::Title;

#[derive(Routable, Copy, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: i64 },
}

impl Route {
    pub fn get_title(&self) -> &'static str {
        match self {
            Route::Home => "Home",
            Route::Blog => "Blog",
            Route::BlogPost { .. } => "Blog" // TODO: fetch title
        }
    }

    pub fn get_html(self) -> Html {
        match self {
            Route::Home => pages::home::render(),
            Route::Blog => pages::blog::render(),
            Route::BlogPost { .. } => pages::blog_post::render(),
        }
    }

    pub fn switch(self) -> Html {
        let title = Title::new_rc(self.get_title());
        let html = self.get_html();

        html! {
            <>
                <Artifact<Title> value={title} />
                { html }
            </>
        }
    }
}
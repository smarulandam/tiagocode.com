use dioxus::prelude::*;

mod adapters;

use crate::adapters::layouts::AppLayout;
use crate::adapters::pages::{ArticleDetailPage, BlogCategoryPage};
use crate::adapters::pages::{BlogListPage, NotFoundPage, PortfolioPage};
use ui::{CUSTOM_CSS, FAVICON};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    #[redirect("/en", || Route::PortfolioPage { lang: "en".to_string() })]

    #[route("/:lang")]
    PortfolioPage { lang: String },

    #[route("/:lang/articles")]
    BlogListPage { lang: String },

    #[route("/:lang/articles/:category")]
    BlogCategoryPage { lang: String, category: String },

    #[route("/:lang/articles/:category/:slug")]
    ArticleDetailPage {
        lang: String,
        category: String,
        slug: String,
    },

    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: CUSTOM_CSS }

        Router::<Route> {}
    }
}

fn main() {
    launch(App);
}

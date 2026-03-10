use dioxus::prelude::*;

mod adapters;

use crate::adapters::layouts::AppLayout;
use crate::adapters::pages::{BlogCategoryPage, BlogDetailPage, BlogListPage};
use crate::adapters::pages::{NotFoundPage, PortfolioPage};
use ui::{CUSTOM_CSS, FAVICON, TAILWIND_CSS};

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
    BlogDetailPage {
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
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: CUSTOM_CSS }

        Router::<Route> {}
    }
}

fn main() {
    launch(App);
}

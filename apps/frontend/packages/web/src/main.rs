use dioxus::prelude::*;

use views::layouts::AppLayout;
use views::pages::{
    ArticleDetailPage, BlogCategoryPage, BlogListPage, NotFoundPage, PortfolioPageEn,
    PortfolioPageEs, RootRedirectPage,
};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    RootRedirectPage {},

    #[route("/en")]
    PortfolioPageEn {},

    #[route("/es")]
    PortfolioPageEs {},

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

pub(crate) const FAVICON: Asset = asset!("/assets/favicon.ico");
pub(crate) const LOGO_TEAL: Asset = asset!("/assets/images/logo_teal.svg");
pub(crate) const MAIN_CSS: Asset = asset!("/assets/main.css");
pub(crate) const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub(crate) const CUSTOM_CSS: Asset = asset!("/assets/css/custom.css");
pub(crate) const BOOTSTRAP_ICONS_CSS: Asset =
    asset!("/assets/plugins/bootstrap-icons/bootstrap-icons.css");
pub(crate) const HIGHLIGHT_CSS: Asset = asset!("/assets/plugins/highlightjs/default.min.css");
pub(crate) const SPLIDE_CSS: Asset = asset!("/assets/plugins/splidejs/css/splide.min.css");
pub(crate) const HIGHLIGHT_JS: Asset = asset!("/assets/plugins/highlightjs/highlight.min.js");
pub(crate) const HIGHLIGHT_LINES_JS: Asset =
    asset!("/assets/plugins/highlightjs/highlightjs-line-numbers.min.js");
pub(crate) const SPLIDE_JS: Asset = asset!("/assets/plugins/splidejs/js/splide.min.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: CUSTOM_CSS }
        document::Stylesheet { href: BOOTSTRAP_ICONS_CSS }
        document::Stylesheet { href: HIGHLIGHT_CSS }
        document::Stylesheet { href: SPLIDE_CSS }
        document::Script { src: HIGHLIGHT_JS }
        document::Script { src: HIGHLIGHT_LINES_JS }
        document::Script { src: SPLIDE_JS }

        Router::<Route> {}
    }
}

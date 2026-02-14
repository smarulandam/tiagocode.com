use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_meta::Stylesheet;
use leptos_router::components::{Redirect, Route, Router, Routes};
use leptos_router::{path, SsrMode, WildcardSegment};

use crate::adapters::driver::leptos_webui::views::pages::{
    BlogDetailPage, BlogListPage, NotFoundPage, PortfolioPage,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Router>
            <Stylesheet id="leptos" href="/pkg/website.css"/>
            <Stylesheet href="/assets/css/custom.css"/>
            <Stylesheet href="/assets/plugins/highlightjs/default.min.css"/>
            <Stylesheet href="/assets/plugins/bootstrap-icons/bootstrap-icons.min.css"/>
            <Routes fallback=move || "Not found.">
                <Route ssr=SsrMode::Async path=path!("/") view=|| view! { <Redirect path="/en" /> } />
                <Route ssr=SsrMode::Async path=path!("/en") view=PortfolioPage/>
                <Route ssr=SsrMode::Async path=path!("/es") view=PortfolioPage/>
                <Route ssr=SsrMode::Async path=path!("/:lang/articles") view=BlogListPage/>
                <Route ssr=SsrMode::Async path=path!("/:lang/articles/:category") view=BlogListPage/>
                <Route ssr=SsrMode::Async path=path!("/:lang/articles/:category/:slug") view=BlogDetailPage/>
                <Route ssr=SsrMode::Async path=WildcardSegment("any") view=NotFoundPage/>
            </Routes>
        </Router>
    }
}

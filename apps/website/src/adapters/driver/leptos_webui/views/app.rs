use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{ParentRoute, Redirect, Route, Router, Routes};
use leptos_router::{path, SsrMode, WildcardSegment};

use crate::adapters::driver::leptos_webui::views::layouts::SiteLayout;
use crate::adapters::driver::leptos_webui::views::pages::{
    BlogDetailPage, BlogListPage, NotFoundPage, PortfolioPage,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=move || "Not found.">
                <ParentRoute path=path!("") view=SiteLayout>
                    <Route path=path!("/portfolio/santiago-marulanda") view=|| view! { <Redirect path="/" /> } />
                    <Route path=path!("es/portafolio/santiago-marulanda") view=|| view! { <Redirect path="/es" /> } />
                    <Route ssr=SsrMode::Async path=path!("") view=PortfolioPage />
                    <Route ssr=SsrMode::Async path=path!("articles") view=BlogListPage/>
                    <Route ssr=SsrMode::Async path=path!("articles/:category") view=BlogListPage/>
                    <Route ssr=SsrMode::Async path=path!("articles/:category/:slug") view=BlogDetailPage/>
                    <Route ssr=SsrMode::Async path=path!(":lang") view=PortfolioPage />
                    <Route ssr=SsrMode::Async path=path!(":lang/articulos") view=BlogListPage/>
                    <Route ssr=SsrMode::Async path=path!(":lang/articulos/:category") view=BlogListPage/>
                    <Route ssr=SsrMode::Async path=path!(":lang/articulos/:category/:slug") view=BlogDetailPage/>
                    <Route path=WildcardSegment("any") view=NotFoundPage/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

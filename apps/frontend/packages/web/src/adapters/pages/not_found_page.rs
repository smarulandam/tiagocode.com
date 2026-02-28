use dioxus::prelude::*;

use ui::NotFoundView;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    dioxus_fullstack::FullstackContext::commit_http_status(
        StatusCode::NOT_FOUND,
        Some("Route not found".to_string()),
    );

    rsx! {
        NotFoundView { route: Some(route.join("/")) }
    }
}

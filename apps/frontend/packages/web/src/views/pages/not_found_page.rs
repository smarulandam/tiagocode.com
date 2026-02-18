use dioxus::prelude::*;

use crate::views::components::common::NotFoundError;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    dioxus_fullstack::FullstackContext::commit_http_status(
        StatusCode::NOT_FOUND,
        Some("Route not found".to_string()),
    );

    rsx! {
        NotFoundError { route: Some(route.join("/")) }
    }
}

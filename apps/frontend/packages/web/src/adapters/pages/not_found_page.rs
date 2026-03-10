use dioxus::prelude::*;

use ui::NotFoundError;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    dioxus_fullstack::FullstackContext::commit_http_status(
        StatusCode::NOT_FOUND,
        Some("Route not found".to_string()),
    );

    rsx! {
        div { class: "flex flex-col justify-center gap-6 lg:flex-row lg:gap-8 xl:gap-12",
            NotFoundError { route: Some(route.join("/")) }
        }
    }
}

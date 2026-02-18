use dioxus::prelude::*;

#[component]
pub fn UnexpectedError() -> Element {
    rsx! {
        section { class: "section-panel border-red-200/80 bg-red-50/80",
            h2 { class: "text-lg font-semibold text-red-900", "Unexpected Error" }
            p { class: "mt-2 text-sm text-red-800/90",
                "Something went wrong while rendering this page."
            }
        }
    }
}

#[component]
pub fn NotFoundError(route: Option<String>) -> Element {
    let route = route.unwrap_or_else(|| "unknown".to_string());

    rsx! {
        section { class: "section-panel border-amber-200/80 bg-amber-50/80",
            h2 { class: "text-lg font-semibold text-amber-900", "404 - Not Found" }
            p { class: "mt-2 text-sm text-amber-900/80", "Route: /{route}" }
        }
    }
}

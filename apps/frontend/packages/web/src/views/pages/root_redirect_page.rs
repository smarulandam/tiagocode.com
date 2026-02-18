use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn RootRedirectPage() -> Element {
    let navigator = use_navigator();

    use_effect(move || {
        navigator.replace(Route::PortfolioPageEn {});
    });

    rsx! {
        section {
            class: "hero-panel",
            p { class: "eyebrow", "Redirecting" }
            p { class: "muted-lead mt-3", "Sending you to /en..." }
        }
    }
}

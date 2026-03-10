use dioxus::prelude::*;

#[component]
pub fn RawHtml(html: String, class: String) -> Element {
    rsx! {
        div {
            class,
            dangerous_inner_html: html,
        }
    }
}

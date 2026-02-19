use dioxus::prelude::*;

#[component]
pub fn RawHtml(html: String, class: Option<String>) -> Element {
    rsx! {
        div {
            class: class.unwrap_or_else(|| "text-base leading-8 text-foreground/85".to_string()),
            dangerous_inner_html: html,
        }
    }
}

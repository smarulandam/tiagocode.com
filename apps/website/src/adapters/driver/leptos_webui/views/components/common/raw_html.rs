use leptos::prelude::*;

#[component]
pub fn RawHtml(html: String, class: &'static str) -> impl IntoView {
    view! {
        <div class=class inner_html=html />
    }
}

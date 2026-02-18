use dioxus::prelude::*;

const CONTAINER_CLASS: &str =
    "section rounded-2xl border border-border bg-card px-6 py-8 md:px-8 md:py-10 lg:p-12";

#[component]
pub fn Container(children: Element, id: Option<String>, class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| CONTAINER_CLASS.to_string());
    let id = id.unwrap_or_default();

    rsx! {
        div { id, class, {children} }
    }
}

#[component]
pub fn PrimaryTitle(text: String) -> Element {
    rsx! {
        h1 { class: "section-heading mb-2 text-4xl font-display font-semibold text-primary", "{text}" }
    }
}

#[component]
pub fn SecondaryTitle(text: String) -> Element {
    rsx! {
        h2 { class: "section-heading mb-2 text-4xl font-display font-semibold text-primary", "{text}" }
    }
}

#[component]
pub fn Decoration(text: String) -> Element {
    rsx! {
        span { class: "section-eyebrow relative mb-5 pt-4 font-mono text-sm font-medium uppercase tracking-wider text-accent before:pr-2 before:content-['//']",
            "{text}"
        }
    }
}

#[component]
pub fn Description(text: String) -> Element {
    rsx! {
        p { class: "section-description text-zeus", "{text}" }
    }
}

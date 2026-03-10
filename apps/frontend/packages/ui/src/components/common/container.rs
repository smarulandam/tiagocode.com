use dioxus::prelude::*;

const CONTAINER_CLASS: &str =
    "relative overflow-hidden rounded-2xl border border-[#d6dee8] bg-white px-6 py-8 shadow-[0_18px_34px_-28px_rgba(17,28,42,0.32)] transition duration-200 ease-out hover:-translate-y-[2px] hover:shadow-[0_24px_42px_-30px_rgba(17,28,42,0.4)] md:px-8 md:py-10 lg:p-12";

#[component]
pub fn Container(children: Element, id: Option<String>, class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| CONTAINER_CLASS.to_string());
    let id = id.unwrap_or_default();

    rsx! {
        div {
            id,
            class,
            {children}
        }
    }
}

#[component]
pub fn PrimaryTitle(text: String) -> Element {
    rsx! {
        h1 {
            class: "mb-2 font-display text-4xl font-semibold tracking-[-0.018em] text-[#087f8c]",
            "{text}"
        }
    }
}

#[component]
pub fn SecondaryTitle(text: String) -> Element {
    rsx! {
        h2 {
            class: "mb-2 font-display text-4xl font-semibold tracking-[-0.018em] text-[#087f8c]",
            "{text}"
        }
    }
}

#[component]
pub fn Decoration(text: String) -> Element {
    rsx! {
        span {
            class: "relative mb-5 pt-4 font-mono text-sm font-medium uppercase tracking-[0.12em] text-[#bb9f06] before:pr-2 before:content-['//']",
            "{text}"
        }
    }
}

#[component]
pub fn Description(text: String) -> Element {
    rsx! {
        p {
            class: "max-w-[68ch] leading-[1.78] text-[#242424]",
            "{text}"
        }
    }
}

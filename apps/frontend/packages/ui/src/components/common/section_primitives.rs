use dioxus::prelude::*;

const SECTION_CONTAINER_CLASS: &str =
    "section bg-white px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-smoke-shadow hover:shadow-smoke-shadow-hover transition ease-out duration-[160ms] rounded-lg";

#[component]
pub fn SectionContainer(
    children: Element,
    #[props(default = String::new())] id: String,
    #[props(default = SECTION_CONTAINER_CLASS.to_string())] class: String,
) -> Element {
    rsx! {
        div { id, class, {children} }
    }
}

#[component]
pub fn PrimarySectionTitle(text: String) -> Element {
    rsx! {
        h1 { class: "text-4xl font-poppins font-semibold mb-2 text-teal", "{text}" }
    }
}

#[component]
pub fn SectionTitle(text: String) -> Element {
    rsx! {
        h2 { class: "text-4xl font-poppins font-semibold mb-2 text-teal", "{text}" }
    }
}

#[component]
pub fn SectionEyebrow(text: String) -> Element {
    rsx! {
        span {
            class: "font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 before:content-['//'] before:pr-2 text-sheengold",
            "{text}"
        }
    }
}

#[component]
pub fn SectionDescription(text: String) -> Element {
    rsx! {
        p { class: "text-zeus", "{text}" }
    }
}

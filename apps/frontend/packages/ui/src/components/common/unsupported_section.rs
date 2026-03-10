use dioxus::prelude::*;

#[component]
pub fn UnsupportedSection() -> Element {
    rsx! {
        p {
            class: "text-center text-2xl font-poppins font-medium text-zeus dark:text-white/70",
            "Missing section"
        }
    }
}

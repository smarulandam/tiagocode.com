use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        a {
            href: "/",
            target: "_self",
            class: "flex items-center gap-3",
            img {
                src: asset!("/assets/images/logo_teal.svg"),
                class: "h-8",
                alt: "Tiagocode Logo",
            }
            span {
                class: "hidden self-center whitespace-nowrap text-2xl font-semibold uppercase tracking-widest text-teal md:block",
                "Tiagocode"
            }
        }
    }
}

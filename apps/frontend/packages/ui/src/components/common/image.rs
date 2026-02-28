use dioxus::prelude::*;

use content_core::application::domain::common::Image;

#[component]
pub fn Img(image: Image, class: Option<String>) -> Element {
    rsx! {
        img {
            class: class
                .unwrap_or_else(|| {
                    "w-full rounded-2xl border border-border/70 shadow-[var(--shadow-card)]"
                        .to_string()
                }),
            src: image.url().to_string(),
            alt: image.alt().to_string(),
            title: image.title().to_string(),
            loading: "lazy",
        }
    }
}

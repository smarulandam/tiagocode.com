use dioxus::prelude::*;

use content_core::application::domain::article::ArticleContent;

use crate::views::components::common::{Img, MissingSection, RawHtml, Slider};

#[component]
pub fn DynamicContent(content: Vec<ArticleContent>) -> Element {
    rsx! {
        for block in content {
            match block {
                ArticleContent::Image(image) => rsx! {
                    Img {
                        image,
                        class: Some(
                            "mt-6 w-full rounded-2xl border border-border/70 shadow-[var(--shadow-card)]"
                                .to_string(),
                        ),
                    }
                },
                ArticleContent::Text(text) => rsx! {
                    RawHtml { html: text.to_string(), class: Some("article-prose mt-6".to_string()) }
                },
                ArticleContent::Slider(thumbnails, images) => rsx! {
                    Slider { thumbnails, images }
                },
                ArticleContent::Unknown => rsx! {
                    MissingSection {}
                },
            }
        }
    }
}

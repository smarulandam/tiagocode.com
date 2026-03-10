use dioxus::prelude::*;

use crate::components::common::{Decoration, PrimaryTitle};
use content_core::application::domain::article::Article;

#[component]
pub fn Header(article: Article) -> Element {
    let category = article.category().clone();
    let published_at = article.created_at().to_string_with_format("%b %d, %Y");

    rsx! {
        header {
            class: "mb-6 border-b border-border/70 pb-6 md:mb-8 md:pb-8",
            div {
                class: "flex flex-wrap items-center justify-between gap-3",
                div {
                    class: "inline-flex items-center rounded-full border border-border/90 bg-surface-soft/75 px-3 py-1.5 font-mono text-xs font-semibold uppercase tracking-[0.1em] text-primary",
                    span { "{category.title()}" }
                    span {
                        class: "ml-2 text-sm",
                        "{category.emoji()}"
                    }
                }
                time {
                    class: "inline-flex items-center rounded-full border border-border/80 bg-white px-3 py-1.5 font-mono text-[0.7rem] font-medium uppercase tracking-[0.1em] text-muted-foreground",
                    "Published {published_at}"
                }
            }

            div {
                class: "mt-5 md:mt-7",
                Decoration { text: "Article detail".to_string() }
                PrimaryTitle { text: article.title().to_string() }
            }
        }
    }
}

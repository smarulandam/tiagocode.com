use dioxus::prelude::*;

use crate::views::components::common::{Decoration, PrimaryTitle};
use content_core::application::domain::article::Article;

#[component]
pub fn Header(article: Article) -> Element {
    let category = article.category().clone();

    rsx! {
        header {
            div { class: "grid grid-cols-1 gap-6 md:grid-cols-2",
                div { class: "category",
                    "{category.title()}"
                    span { class: "ml-2", "{category.emoji()}" }
                }
                time { class: "category md:text-right",
                    "Published at"
                    span { class: "ml-1", "{article.created_at().to_string_with_format(\"%b %d, %Y\")}" }
                }
            }

            div { class: "my-6 md:my-10",
                Decoration { text: "Article detail".to_string() }
                PrimaryTitle { text: article.title().to_string() }
            }
        }
    }
}

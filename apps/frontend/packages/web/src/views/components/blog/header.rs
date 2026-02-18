use dioxus::prelude::*;

use content_core::application::domain::article::Article;

#[component]
pub fn Header(article: Article) -> Element {
    let category = article.category().clone();

    rsx! {
        header {
            div {
                class: "grid grid-cols-1 gap-6 md:grid-cols-2",
                div {
                    class: "category",
                    "{category.title()}"
                    span { class: "ml-2", "{category.emoji()}" }
                }
                time {
                    class: "category md:text-right",
                    "Published at"
                    span { class: "ml-1", "{article.created_at().to_string_with_format(\"%b %d, %Y\")}" }
                }
            }

            div {
                class: "my-6 md:my-10",
                span {
                    class: "font-mono relative mb-5 pt-4 text-sm font-medium tracking-wider text-accent before:pr-2 before:content-['//']",
                    "Article detail"
                }
                h1 { class: "mb-2 text-4xl font-display font-semibold text-primary", "{article.title()}" }
            }
        }
    }
}

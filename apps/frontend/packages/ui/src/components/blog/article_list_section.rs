use dioxus::prelude::*;

use super::ArticleCard;
use crate::components::common::{Pill, PrimarySectionTitle, SectionContainer, SectionEyebrow};
use content_core::application::domain::article::{Article, Category};

#[component]
pub fn ArticleListSection(categories: Vec<Category>, articles: Vec<Article>) -> Element {
    let are_articles_empty = articles.is_empty();

    rsx! {
        SectionContainer {
            div {
                SectionEyebrow { text: "My Tech Articles".to_string() }
                PrimarySectionTitle { text: "Blog".to_string() }
                div { class: "py-6",
                    Pill {
                        link: "/en/articles".to_string(),
                        text: "All".to_string(),
                    }
                    for category in categories {
                        Pill {
                            link: category.slug().to_string(),
                            text: category.title().to_string(),
                            emoji: category.emoji().to_string(),
                        }
                    }
                }
            }
            if !are_articles_empty {
                div { class: "mt-6 grid grid-cols-1 gap-x-6 gap-y-8 lg:mt-8 lg:grid-cols-2 xl:grid-cols-3",
                    for article in articles {
                        ArticleCard {
                            date: article.created_at().to_string_with_format("%b %d, %Y"),
                            title: article.title().to_string(),
                            summary: article.summary().to_string(),
                            slug: article.slug().to_string(),
                            category: article.category().clone(),
                            thumbnail: article.thumbnail().clone(),
                        }
                    }
                }
            } else {
                p {
                    class: "font-mono font-medium uppercase text-sm text-center tracking-wider relative pt-4 mb-5 text-asparagus",
                    "No articles available. Check back soon!"
                }
            }
        }
    }
}

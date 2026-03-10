use dioxus::prelude::*;

use super::FeaturedArticleCard;
use crate::components::common::{
    SectionContainer, SectionDescription, SectionEyebrow, SectionTitle,
};
use content_core::application::domain::article::Article;

#[component]
pub fn FeaturedArticlesSection(
    title: String,
    subtitle: String,
    text: String,
    articles: Vec<Article>,
) -> Element {
    let are_articles_empty = articles.is_empty();

    rsx! {
        SectionContainer { id: "blog".to_string(),
            div {
                SectionEyebrow { text: subtitle }
                SectionTitle { text: title }
                SectionDescription { text }
            }
            div { class: "mt-6 flex flex-col gap-6 lg:mt-10",
                if are_articles_empty {
                    p {
                        class: "font-mono font-medium uppercase text-sm text-center tracking-wider relative pt-4 mb-5 text-asparagus",
                        "No articles available. Check back soon!"
                    }
                } else {
                    for article in articles {
                        FeaturedArticleCard {
                            published_at: article.created_at().to_string_with_format("%b %d, %Y"),
                            title: article.title().to_string(),
                            summary: article.summary().to_string(),
                            link: article.slug().to_string(),
                            category: format!(
                                "{} {}",
                                article.category().title().to_string(),
                                article.category().emoji().to_string(),
                            ),
                            thumbnail: article.thumbnail().clone(),
                        }
                    }
                    div { class: "flex justify-center pt-6",
                        a {
                            href: "/en/articles",
                            target: "_self",
                            aria_label: "View all articles",
                            class: "inline-flex items-center justify-center border-b border-teal/35 px-1 pb-1 pt-4 text-base font-medium text-teal transition duration-[120ms] ease-out hover:border-teal hover:text-deepsea focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal/25 focus-visible:ring-offset-2",
                            "View All Articles"
                        }
                    }
                }
            }
        }
    }
}

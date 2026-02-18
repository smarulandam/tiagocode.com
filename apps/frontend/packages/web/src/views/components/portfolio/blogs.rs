use dioxus::prelude::*;

use crate::views::components::common::{Container, Decoration, Description, Img, SecondaryTitle};
use content_core::application::domain::common::Image;
use content_core::application::domain::portfolio::Blogs;

#[component]
pub fn BlogsSection(data: Blogs, lang: String) -> Element {
    let _ = lang;
    let are_articles_empty = data.articles().is_empty();

    rsx! {
        Container { id: Some("blog".to_string()),

            div {
                Decoration { text: data.subtitle().to_string() }
                SecondaryTitle { text: data.title().to_string() }
                Description { text: data.text().to_string() }
            }

            div { class: "mt-6 space-y-4 lg:mt-10",
                if are_articles_empty {
                    p { class: "relative mb-5 pt-4 text-center font-mono text-sm font-medium uppercase tracking-wider text-primary",
                        "No articles available. Check back soon!"
                    }
                } else {
                    for article in data.articles().iter() {
                        FeaturedArticleCard {
                            published_at: article.created_at().to_string_with_format("%b %d, %Y"),
                            title: article.title().to_string(),
                            summary: article.summary().to_string(),
                            link: article.slug().to_string(),
                            thumbnail: article.thumbnail().clone(),
                            category: format!(
                                "{} {}",
                                article.category().title().to_string(),
                                article.category().emoji().to_string(),
                            ),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FeaturedArticleCard(
    published_at: String,
    title: String,
    summary: String,
    link: String,
    thumbnail: Image,
    category: String,
) -> Element {
    rsx! {
        article { class: "featured-article sm:flex md:items-start",
            div { class: "relative overflow-hidden rounded-lg group flex-shrink-0",
                Img {
                    image: thumbnail,
                    class: Some(
                        "w-full sm:max-w-[340px] transition duration-500 ease-out group-hover:scale-105"
                            .to_string(),
                    ),
                }
                div { class: "article-badge absolute left-4 top-4 rounded-full px-4 py-2 font-mono text-xs font-medium uppercase tracking-wider text-white backdrop-blur-sm",
                    "{category}"
                }
            }

            div { class: "mt-5 flex-grow md:mt-0 md:pl-7",
                span { class: "article-date text-zeus", "Posted on {published_at}" }
                h3 { class: "mt-2 font-display text-2xl font-semibold text-primary", "{title}" }
                p { class: "article-summary text-zeus", "{summary}" }
                a {
                    href: link,
                    target: "_self",
                    class: "button-link mt-3 inline-flex items-center lg:mt-4",
                    "Read More"
                }
            }
        }
    }
}

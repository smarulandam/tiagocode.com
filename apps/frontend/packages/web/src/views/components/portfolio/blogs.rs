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

            div { class: "mt-6 space-y-8 md:space-y-6 lg:mt-12",
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
        article { class: "sm:flex md:items-start",
            div { class: "relative overflow-hidden rounded-lg group flex-shrink-0",
                Img {
                    image: thumbnail,
                    class: Some(
                        "w-full sm:max-w-[340px] transition duration-500 ease-out group-hover:scale-105 group-hover:blur-[1.5px]"
                            .to_string(),
                    ),
                }
                div { class: "absolute left-4 top-4 rounded-full bg-black/20 px-4 py-2 font-mono text-sm font-normal uppercase tracking-[0.5px] text-white backdrop-blur-[5px]",
                    "{category}"
                }
            }

            div { class: "mt-5 flex-grow md:mt-0 md:pl-7",
                span { class: "text-zeus", "Posted on {published_at}" }
                h3 { class: "mt-2 font-display text-2xl font-semibold", "{title}" }
                p { class: "text-zeus", "{summary}" }
                a {
                    href: link,
                    target: "_self",
                    class: "mt-3 inline-block rounded-full border border-black border-dashed px-6 py-3 font-mono text-sm transition duration-[120ms] ease-out hover:bg-black hover:text-white lg:mt-4",
                    "Read More"
                }
            }
        }
    }
}

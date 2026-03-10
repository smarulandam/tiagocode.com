use dioxus::prelude::*;

use crate::components::common::{Container, Decoration, Description, Img, SecondaryTitle};
use content_core::application::domain::common::Image;
use content_core::application::domain::portfolio::Blogs;

#[component]
pub fn BlogsSection(data: Blogs) -> Element {
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
        article { class: "rounded-2xl border border-[#d6dee8] bg-white p-4 shadow-[0_14px_26px_-22px_rgba(17,28,42,0.26)] transition duration-200 ease-out hover:-translate-y-[2px] hover:border-accent/30 hover:shadow-[0_20px_34px_-26px_rgba(17,28,42,0.32)] sm:flex md:items-start",
            div { class: "relative overflow-hidden rounded-lg group flex-shrink-0",
                Img {
                    image: thumbnail,
                    class: Some(
                        "w-full sm:max-w-[340px] transition duration-500 ease-out group-hover:scale-105"
                            .to_string(),
                    ),
                }
                div { class: "absolute left-4 top-4 rounded-full border border-white/30 bg-slate-900/60 px-4 py-2 font-mono text-xs font-medium uppercase tracking-wider text-white shadow-[0_8px_16px_-14px_rgba(8,15,30,0.72)] backdrop-blur-sm",
                    "{category}"
                }
            }

            div { class: "mt-5 flex-grow md:mt-0 md:pl-7",
                span { class: "font-mono text-[0.72rem] uppercase tracking-[0.08em] text-muted-foreground", "Posted on {published_at}" }
                h3 { class: "mt-2 font-display text-2xl font-semibold text-primary", "{title}" }
                p { class: "leading-[1.74] text-zeus", "{summary}" }
                a {
                    href: link,
                    target: "_self",
                    class: "mt-3 inline-flex items-center justify-center rounded-xl border border-border/90 bg-surface-soft/60 px-4 py-2 font-mono text-xs font-semibold uppercase tracking-[0.09em] text-primary transition duration-150 ease-out hover:-translate-y-px hover:border-accent/40 hover:bg-accent/10 hover:text-accent lg:mt-4",
                    "Read More"
                }
            }
        }
    }
}

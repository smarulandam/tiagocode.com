use dioxus::prelude::*;

use crate::views::components::common::{Container, Decoration, Img, PrimaryTitle};
use content_core::application::domain::article::{Article, Category};

#[component]
pub fn ListSection(
    categories: Vec<Category>,
    articles: Vec<Article>,
    lang: String,
    selected_category: Option<String>,
) -> Element {
    const PILL_BASE_CLASS: &str =
        "inline-flex items-center rounded-full border border-border/90 bg-surface-soft/60 px-4 py-2 font-mono text-sm text-zeus transition duration-150 ease-linear hover:border-accent/45 hover:bg-accent/10 hover:text-primary";
    let are_articles_empty = articles.is_empty();
    let all_articles_url = format!("/{lang}/articles");

    rsx! {
        Container {

            div {
                Decoration { text: "My Tech Articles".to_string() }
                PrimaryTitle { text: "Blog".to_string() }

                div { class: "py-6",
                    a {
                        class: if selected_category.is_none() {
                            format!("{PILL_BASE_CLASS} border-accent/45 bg-accent/15 text-primary")
                        } else {
                            PILL_BASE_CLASS.to_string()
                        },
                        href: all_articles_url,
                        target: "_self",
                        span { class: "inline-block font-mono text-sm", "All" }
                    }

                    for category in categories {
                        a {
                            class: if selected_category
    .as_ref()
    .map(|value| category.slug().to_string().ends_with(value))
    .unwrap_or(false) {
                                format!("{PILL_BASE_CLASS} border-accent/45 bg-accent/15 text-primary")
                            } else {
                                PILL_BASE_CLASS.to_string()
                            },
                            target: "_self",
                            href: category.slug().to_string(),
                            span { class: "mr-2", "{category.emoji()}" }
                            span { class: "inline-block font-mono text-sm", "{category.title()}" }
                        }
                    }
                }
            }

            if !are_articles_empty {
                div { class: "mt-6 grid grid-cols-1 gap-4 lg:mt-3 xl:grid-cols-2",
                    for article in articles {
                        ArticleCard { article }
                    }
                }
            } else {
                p { class: "relative mb-5 pt-4 text-center font-mono text-sm font-medium uppercase tracking-wider text-zeus",
                    "No articles available. Check back soon!"
                }
            }
        }
    }
}

#[component]
fn ArticleCard(article: Article) -> Element {
    let date = article.created_at().to_string_with_format("%b %d, %Y");
    let slug = article.slug().to_string();
    let title = article.title().to_string();
    let summary = truncate_summary(article.summary().to_string(), 110);
    let thumbnail = article.thumbnail().clone();
    let category = article.category().clone();
    let category_slug = category.slug().to_string();
    let category_title = category.title().to_string();
    let category_emoji = category.emoji().to_string();

    rsx! {
        article { class: "rounded-2xl border border-[#d6dee8] bg-white p-4 shadow-[0_14px_26px_-22px_rgba(17,28,42,0.26)] transition duration-200 ease-out hover:-translate-y-[2px] hover:border-accent/30 hover:shadow-[0_20px_34px_-26px_rgba(17,28,42,0.32)] sm:flex sm:items-start",
            div { class: "group relative w-full overflow-hidden rounded-lg md:w-[280px] md:flex-shrink-0",
                Img {
                    image: thumbnail,
                    class: "w-full transition duration-500 ease-out group-hover:scale-105 group-hover:blur-[1.5px]"
                }
                div { class: "absolute bottom-0 left-0 right-0 rounded-none bg-slate-900/55 px-4 py-3 text-center font-mono text-sm font-semibold tracking-[0.04em] text-white backdrop-blur-[5px]",
                    a { href: category_slug, target: "_self",
                        "{category_title}"
                        span { class: "ml-2", "{category_emoji}" }
                    }
                }
            }
            div { class: "mt-4 flex-grow md:mt-0 md:pl-7",
                span { class: "font-mono text-[0.72rem] uppercase tracking-[0.08em] text-muted-foreground", "{date}" }
                h2 { class: "mt-2 font-display text-lg font-semibold text-primary",
                    "{title}"
                }
                p { class: "leading-[1.74] text-zeus", "{summary}" }
                a {
                    href: slug,
                    target: "_self",
                    class: "mt-3 inline-flex items-center justify-center rounded-xl border border-border/90 bg-surface-soft/60 px-4 py-2 font-mono text-xs font-semibold uppercase tracking-[0.09em] text-primary transition duration-150 ease-out hover:-translate-y-px hover:border-accent/40 hover:bg-accent/10 hover:text-accent lg:mt-4",
                    "Read More"
                }
            }
        }
    }
}

fn truncate_summary(summary: String, max_chars: usize) -> String {
    if summary.chars().count() > max_chars {
        format!("{}...", summary.chars().take(max_chars).collect::<String>())
    } else {
        summary
    }
}

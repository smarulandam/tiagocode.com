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
        "inline-flex items-center rounded-full border border-border/90 bg-surface-soft/60 px-4 py-2 font-mono text-[0.74rem] font-medium uppercase tracking-[0.08em] text-zeus transition duration-150 ease-linear hover:border-accent/45 hover:bg-accent/10 hover:text-primary";
    let are_articles_empty = articles.is_empty();
    let all_articles_url = format!("/{lang}/articles");

    rsx! {
        Container {

            div {
                Decoration { text: "My Tech Articles".to_string() }
                PrimaryTitle { text: "Blog".to_string() }

                div { class: "flex flex-wrap items-center gap-3 pt-6",
                    a {
                        class: if selected_category.is_none() {
                            format!("{PILL_BASE_CLASS} border-accent/45 bg-accent/15 text-primary")
                        } else {
                            PILL_BASE_CLASS.to_string()
                        },
                        href: all_articles_url,
                        target: "_self",
                        span { "All" }
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
                            span { class: "mr-2 text-sm", "{category.emoji()}" }
                            span { "{category.title()}" }
                        }
                    }
                }
            }

            if !are_articles_empty {
                div { class: "mt-7 grid grid-cols-1 gap-5 lg:mt-5 lg:gap-6 xl:grid-cols-2",
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
        article { class: "group relative h-full overflow-hidden rounded-[1.35rem] border border-[#d6dee8] bg-[linear-gradient(180deg,#ffffff_0%,#f9fcff_100%)] p-4 shadow-[0_14px_26px_-22px_rgba(17,28,42,0.26)] transition duration-200 ease-out hover:-translate-y-[2px] hover:border-accent/30 hover:shadow-[0_22px_36px_-26px_rgba(17,28,42,0.34)] before:absolute before:inset-x-0 before:top-0 before:h-[1px] before:bg-gradient-to-r before:from-cyan-300/40 before:via-teal-700/35 before:to-cyan-300/35 before:content-[''] sm:flex sm:items-stretch sm:gap-5",
            div { class: "relative w-full overflow-hidden rounded-xl border border-border/75 bg-slate-100 sm:w-[265px] sm:flex-shrink-0",
                Img {
                    image: thumbnail,
                    class: "h-full w-full object-cover transition duration-500 ease-out group-hover:scale-105"
                }
                div { class: "pointer-events-none absolute inset-x-0 bottom-0 h-24 bg-gradient-to-t from-slate-900/45 to-transparent" }
                a {
                    href: category_slug,
                    target: "_self",
                    class: "absolute left-3 top-3 inline-flex items-center rounded-full border border-white/65 bg-slate-900/60 px-3 py-1 font-mono text-[0.68rem] font-semibold uppercase tracking-[0.08em] text-white backdrop-blur-md transition duration-150 ease-out hover:border-white/85 hover:bg-slate-900/78",
                    "{category_title}"
                    span { class: "ml-1.5 text-[0.82rem]", "{category_emoji}" }
                }
            }
            div { class: "mt-4 flex min-w-0 flex-grow flex-col sm:mt-0",
                div { class: "flex items-center gap-2",
                    span { class: "font-mono text-[0.68rem] uppercase tracking-[0.11em] text-muted-foreground", "{date}" }
                    span { class: "h-1 w-1 rounded-full bg-muted-foreground/70" }
                    span { class: "font-mono text-[0.68rem] uppercase tracking-[0.11em] text-muted-foreground", "Article" }
                }
                h2 { class: "mt-2 text-balance font-display text-[1.18rem] font-semibold leading-[1.32] text-primary",
                    "{title}"
                }
                p { class: "mt-3 leading-[1.72] text-zeus/95", "{summary}" }
                div { class: "mt-4 border-t border-border/70 pt-4",
                    a {
                        href: slug,
                        target: "_self",
                        class: "inline-flex items-center justify-center self-start rounded-xl border border-border/90 bg-surface-soft/65 px-4 py-2 font-mono text-xs font-semibold uppercase tracking-[0.09em] text-primary transition duration-150 ease-out hover:-translate-y-px hover:border-accent/40 hover:bg-accent/10 hover:text-accent",
                        "Read More"
                        span { class: "ml-2 text-[0.9rem] transition-transform duration-150 ease-out group-hover:translate-x-0.5",
                            "→"
                        }
                    }
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

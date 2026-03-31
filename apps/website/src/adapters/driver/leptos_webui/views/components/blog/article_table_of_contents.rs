use lazy_static::lazy_static;
use leptos::either::Either;
use leptos::prelude::*;
use regex::Regex;
use voca_rs::strip::strip_tags;

use crate::application::domain::article::{ArticleContent, ArticleContentTableItem};

lazy_static! {
    static ref ARTICLE_SECTION_HEADING_REGEX: Regex =
        Regex::new(r#"(?is)<h([1-3])([^>]*)>(.*?)</h([1-3])>"#).unwrap();
    static ref HEADING_ID_ATTRIBUTE_REGEX: Regex =
        Regex::new(r#"(?i)\bid\s*=\s*["']([^"']+)["']"#).unwrap();
}

#[component]
pub fn ArticleContentTable(items: Vec<ArticleContentTableItem>) -> impl IntoView {
    let desktop_table_of_contents_items = items.clone();
    let mobile_table_of_contents_items = items.clone();
    let total_table_of_contents_items = items.len();

    let (is_mobile_table_of_contents_open, set_is_mobile_table_of_contents_open) = signal(false);

    if total_table_of_contents_items == 0 {
        Either::Left(view! { <></> })
    } else {
        Either::Right(view! {
            <>
                <aside class="hidden lg:sticky mt-7 md:mt-8 lg:top-[108px] lg:block lg:w-[280px] lg:self-start xl:w-[300px]">
                    <div class="article-table-of-contents-panel">
                        <p class="article-table-of-contents-label">"On this page"</p>
                        <nav aria-label="Article table of contents" class="article-table-of-contents-list">
                            {desktop_table_of_contents_items
                                .into_iter()
                                .enumerate()
                                .map(|(index, item)| {
                                    let level = item.level;
                                    let item_index = index.to_string();
                                    let title = item.title;
                                    let is_disabled = item.id.is_none();
                                    let class = if is_disabled {
                                        format!("article-table-of-contents-item article-table-of-contents-item-level-{level} is-disabled")
                                    } else {
                                        format!("article-table-of-contents-item article-table-of-contents-item-level-{level}")
                                    };

                                    match item.id {
                                        Some(id) => {
                                            let href = format!("#{id}");

                                            Either::Left(view! {
                                                <a href=href class=class data-article-table-of-contents-index=item_index>
                                                    <span>{title}</span>
                                                </a>
                                            })
                                        }
                                        None => Either::Right(view! {
                                            <span class=class data-article-table-of-contents-index=item_index aria-disabled="true">
                                                <span>{title}</span>
                                            </span>
                                        }),
                                    }
                                })
                                .collect_view()}
                        </nav>
                    </div>
                </aside>

                <button
                    type="button"
                    class="article-table-of-contents-button lg:hidden"
                    aria-controls="article-table-of-contents-sheet"
                    aria-expanded=move || if is_mobile_table_of_contents_open.get() { "true" } else { "false" }
                    on:click=move |_| set_is_mobile_table_of_contents_open.set(true)
                >
                    <span>"Sections"</span>
                    <span class="article-table-of-contents-button-badge">{total_table_of_contents_items}</span>
                </button>

                <div
                    class="article-table-of-contents-sheet-overlay lg:hidden"
                    class:hidden=move || !is_mobile_table_of_contents_open.get()
                    on:click=move |_| set_is_mobile_table_of_contents_open.set(false)
                ></div>
                <div
                    class="article-table-of-contents-sheet lg:hidden"
                    class:hidden=move || !is_mobile_table_of_contents_open.get()
                    id="article-table-of-contents-sheet"
                    data-article-table-of-contents-sheet="true"
                    role="dialog"
                    aria-modal="true"
                    aria-label="Article table of contents"
                >
                    <div class="flex items-start justify-between gap-4">
                        <div>
                            <p class="article-table-of-contents-label">"On this page"</p>
                            <p class="mt-2 font-poppins text-[1.55rem] font-semibold leading-[1.08] text-deepsea">
                                "Jump to a section"
                            </p>
                        </div>
                        <button
                            type="button"
                            class="rounded-full border border-black/8 bg-smoke px-3.5 py-2 text-sm font-medium text-zeus/72 transition duration-[120ms] ease-out hover:border-teal/18 hover:bg-white hover:text-deepsea focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal/20 focus-visible:ring-offset-2"
                            on:click=move |_| set_is_mobile_table_of_contents_open.set(false)
                        >
                            "Close"
                        </button>
                    </div>

                    <nav aria-label="Article table of contents" class="article-table-of-contents-sheet-list">
                        {mobile_table_of_contents_items
                            .into_iter()
                            .enumerate()
                            .map(|(index, item)| {
                                let level = item.level;
                                let item_index = index.to_string();
                                let title = item.title;
                                let is_disabled = item.id.is_none();
                                let class = if is_disabled {
                                    format!("article-table-of-contents-item article-table-of-contents-item-level-{level} is-disabled")
                                } else {
                                    format!("article-table-of-contents-item article-table-of-contents-item-level-{level}")
                                };

                                match item.id {
                                    Some(id) => {
                                        let href = format!("#{id}");

                                        Either::Left(view! {
                                            <a
                                                href=href
                                                class=class
                                                data-article-table-of-contents-index=item_index
                                                on:click=move |_| set_is_mobile_table_of_contents_open.set(false)
                                            >
                                                <span>{title}</span>
                                            </a>
                                        })
                                    }
                                    None => Either::Right(view! {
                                        <span class=class data-article-table-of-contents-index=item_index aria-disabled="true">
                                            <span>{title}</span>
                                        </span>
                                    }),
                                }
                            })
                            .collect_view()}
                    </nav>
                </div>
            </>
        })
    }
}

pub fn collect_article_table_of_contents_items(
    content: &[ArticleContent],
) -> Vec<ArticleContentTableItem> {
    let mut table_of_contents_items = Vec::new();

    for content_block in content {
        let ArticleContent::Text(text) = content_block else {
            continue;
        };

        for capture in ARTICLE_SECTION_HEADING_REGEX.captures_iter(text.as_str()) {
            let Some(level_match) = capture.get(1) else {
                continue;
            };

            let Some(closing_level_match) = capture.get(4) else {
                continue;
            };

            if level_match.as_str() != closing_level_match.as_str() {
                continue;
            }

            let Some(attrs_match) = capture.get(2) else {
                continue;
            };

            let Some(inner_html_match) = capture.get(3) else {
                continue;
            };

            let title = extract_heading_text(inner_html_match.as_str());

            if title.is_empty() {
                continue;
            }

            table_of_contents_items.push(ArticleContentTableItem {
                title,
                level: level_match.as_str().parse::<u8>().unwrap_or(2),
                id: extract_heading_anchor_id(attrs_match.as_str()),
            });
        }
    }

    table_of_contents_items
}

fn extract_heading_anchor_id(attrs: &str) -> Option<String> {
    HEADING_ID_ATTRIBUTE_REGEX
        .captures(attrs)
        .and_then(|capture| capture.get(1))
        .map(|value| value.as_str().trim().to_string())
        .filter(|value| !value.is_empty())
}

fn extract_heading_text(value: &str) -> String {
    strip_tags(value)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::value_objects::RequiredText;

    fn text_block(value: &str) -> ArticleContent {
        ArticleContent::Text(RequiredText::try_from(value).unwrap())
    }

    #[test]
    fn collects_headings_from_text_blocks_without_touching_non_text_blocks() {
        let content = vec![
            ArticleContent::Unknown,
            text_block(
                r#"
                    <h2 id="intro"><span>Hello</span> <strong>World</strong> 🧠</h2>
                    <p>Body</p>
                    <h3>Inner topic</h3>
                    <h4>Ignored</h4>
                "#,
            ),
        ];

        let table_of_contents_items = collect_article_table_of_contents_items(&content);

        assert_eq!(table_of_contents_items.len(), 2);
        assert_eq!(table_of_contents_items[0].title, "Hello World 🧠");
        assert_eq!(table_of_contents_items[0].level, 2);
        assert_eq!(table_of_contents_items[0].id.as_deref(), Some("intro"));
        assert_eq!(table_of_contents_items[1].title, "Inner topic");
        assert_eq!(table_of_contents_items[1].level, 3);
        assert_eq!(table_of_contents_items[1].id, None);
    }

    #[test]
    fn keeps_headings_without_ids_visible_in_the_table_of_contents() {
        let content = vec![text_block(
            "<h2>Overview</h2><h3 id=\"details\">Details</h3>",
        )];

        let table_of_contents_items = collect_article_table_of_contents_items(&content);

        assert_eq!(table_of_contents_items.len(), 2);
        assert_eq!(table_of_contents_items[0].title, "Overview");
        assert_eq!(table_of_contents_items[0].id, None);
        assert_eq!(table_of_contents_items[1].title, "Details");
        assert_eq!(table_of_contents_items[1].id.as_deref(), Some("details"));
    }

    #[test]
    fn preserves_heading_order_across_text_blocks_and_ignores_h4() {
        let content = vec![
            text_block("<h2 id=\"intro\">Intro</h2><h4>Ignore me</h4>"),
            text_block("<h3>Second step</h3><h1 id=\"final\">Final</h1>"),
        ];

        let table_of_contents_items = collect_article_table_of_contents_items(&content);

        assert_eq!(table_of_contents_items.len(), 3);
        assert_eq!(table_of_contents_items[0].title, "Intro");
        assert_eq!(table_of_contents_items[1].title, "Second step");
        assert_eq!(table_of_contents_items[2].title, "Final");
        assert_eq!(table_of_contents_items[2].level, 1);
        assert_eq!(table_of_contents_items[2].id.as_deref(), Some("final"));
    }
}

use leptos::either::Either;
use leptos::prelude::*;

use crate::application::domain::article::ContentTableItem;

#[component]
pub fn ArticleContentTable(items: Vec<ContentTableItem>) -> impl IntoView {
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

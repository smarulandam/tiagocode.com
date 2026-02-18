use dioxus::prelude::*;

use crate::views::components::blog::{DynamicContent, Header};
use crate::views::components::common::MetaTagsView;

#[component]
pub fn ArticleDetailPage(lang: String, category: String, slug: String) -> Element {
    let full_slug = format!("/{lang}/articles/{category}/{slug}");

    let data = use_loader(move || {
        let full_slug = full_slug.clone();
        async move { api::article_detail_controller(full_slug).await }
    })?;

    let article = data.read().clone();
    let init_script = r#"
        (() => {
            if (window.hljs) {
                document.querySelectorAll('pre code').forEach((block) => {
                    if (block.dataset.hljsProcessed === 'true') {
                        return;
                    }

                    window.hljs.highlightElement(block);
                    if (window.hljs.lineNumbersBlock) {
                        window.hljs.lineNumbersBlock(block);
                    }

                    block.dataset.hljsProcessed = 'true';
                });
            }

            if (window.Splide) {
                document.querySelectorAll('[data-slider]').forEach((element) => {
                    const identifier = element.dataset.slider;
                    if (!identifier || element.dataset.splideMounted === 'true') {
                        return;
                    }

                    const mainSelector = `#main-slider-${identifier}`;
                    const thumbnailSelector = `#thumbnail-slider-${identifier}`;
                    const mainElement = document.querySelector(mainSelector);
                    const thumbnailElement = document.querySelector(thumbnailSelector);

                    if (!mainElement || !thumbnailElement) {
                        return;
                    }

                    const main = new window.Splide(mainSelector, {
                        type: 'fade',
                        rewind: true,
                        pagination: false,
                        arrows: false,
                    });

                    const thumbnails = new window.Splide(thumbnailSelector, {
                        fixedWidth: 100,
                        fixedHeight: 60,
                        gap: 10,
                        rewind: true,
                        pagination: false,
                        isNavigation: true,
                        focus: 'center',
                        breakpoints: {
                            600: {
                                fixedWidth: 60,
                                fixedHeight: 44,
                            },
                        },
                    });

                    main.sync(thumbnails);
                    main.mount();
                    thumbnails.mount();

                    element.dataset.splideMounted = 'true';
                });
            }
        })();
    "#;

    rsx! {
        MetaTagsView { metatags: article.metatags().clone() }

        script {
            dangerous_inner_html: init_script,
        }

        div {
            class: "justify-center space-y-6 lg:flex lg:space-y-0 lg:space-x-8 xl:space-x-12",
            div {
                class: "article-detail section rounded-lg bg-white px-6 py-8 pb-12 shadow-[var(--shadow-panel)] transition duration-[160ms] ease-out hover:shadow-[var(--shadow-soft)] md:px-8 md:py-10 lg:w-3/4 lg:p-12",
                Header { article: article.clone() }
                DynamicContent { content: article.content().clone() }
            }
        }
    }
}

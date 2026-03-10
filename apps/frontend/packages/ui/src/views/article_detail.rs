use dioxus::prelude::*;

use crate::components::blog::{DynamicContent, Header};
use crate::components::common::{Container, MetaTags};
use content_core::application::domain::article::Article;

#[component]
pub fn ArticleDetailView(article: Article) -> Element {
    let script = r#"
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
        Stylesheet { href: asset!("/assets/plugins/highlightjs/default.min.css") }
        Stylesheet { href: asset!("/assets/plugins/splidejs/css/splide.min.css") }

        MetaTags {
            metatags: article.metatags().clone()
        }

        div {
            class: "justify-center pb-12 lg:flex",

            Container {
                class: Some("relative overflow-hidden rounded-2xl border border-[#d6dee8] bg-white px-6 py-8 pb-12 text-[#223044] shadow-[0_18px_34px_-28px_rgba(17,28,42,0.32)] transition duration-200 ease-out hover:-translate-y-[2px] hover:shadow-[0_24px_42px_-30px_rgba(17,28,42,0.4)] before:absolute before:inset-x-0 before:top-0 before:h-[2px] before:bg-gradient-to-r before:from-cyan-300/40 before:via-teal-700/40 before:to-cyan-300/30 before:content-[''] md:px-8 md:py-10 lg:w-3/4 lg:p-12".to_string(),),
                Header {
                    article: article.clone()
                }
                DynamicContent {
                    content: article.content().clone()
                }
            }
        }

        script { src: asset!("/assets/plugins/highlightjs/highlight.min.js") }
        script { src: asset!("/assets/plugins/highlightjs/highlightjs-line-numbers.min.js") }
        script { src: asset!("/assets/plugins/splidejs/js/splide.min.js") }
        script { dangerous_inner_html: script }
    }
}

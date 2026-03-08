use leptos::prelude::*;
use leptos_meta::{Script, Stylesheet};
use leptos_router::hooks::use_location;

use crate::adapters::driver::leptos_webui::controllers::article_detail_controller;
use crate::adapters::driver::leptos_webui::views::components::blog::{DynamicContent, Header};
use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::adapters::driver::leptos_webui::views::layouts::BasicLayout;

#[component]
pub fn BlogDetailPage() -> impl IntoView {
    let route = use_location();
    let page_data = Resource::new(
        move || route.pathname.read().to_string(),
        |slug| article_detail_controller(slug),
    );

    view! {
        <BasicLayout>
            <Suspense fallback=move || { view! { <div>"Loading..."</div> } }>
                {move || {
                    page_data
                    .get_untracked()
                    .map(|data| {
                        if let Err(_) = data {
                            return view! { <UnexpectedError /> }.into_any();
                        }

                        let article = data.unwrap();

                        view! {
                            <MetaTags metatags=article.metatags().clone() />
                            <Stylesheet href="/assets/plugins/splidejs/css/splide.min.css" />
                            <Script src="/assets/plugins/splidejs/js/splide.min.js" />
                            <Script src="/assets/plugins/highlightjs/highlight.min.js" />
                            <Script src="/assets/plugins/highlightjs/highlightjs-line-numbers.min.js" />

                            <div class="pb-12">
                                <div class="article-shell -mx-5 w-auto rounded-none bg-white px-5 py-7 shadow-smoke-shadow transition ease-out duration-[160ms] md:px-8 md:py-10 lg:px-10 lg:py-12 xl:mx-auto xl:w-full xl:rounded-[1.25rem] xl:px-12">
                                    <Header article=article.clone() />
                                    <DynamicContent content=article.content().clone() />
                                </div>
                            </div>

                            <script>
                                "const initArticleDetail = () => {
                                    if (window.hljs) {
                                        const languageAliases = {
                                            sh: 'bash',
                                            shell: 'bash',
                                            zsh: 'bash',
                                            js: 'javascript',
                                            ts: 'typescript',
                                            py: 'python',
                                            rs: 'rust',
                                        };

                                        document.querySelectorAll('pre code, code[class*=\"language-\"]').forEach(function(element) {
                                            if (element.dataset.hljsReady === 'true') {
                                                return;
                                            }

                                            const languageClass = Array.from(element.classList).find(function(className) {
                                                return className.startsWith('language-');
                                            });

                                            if (languageClass) {
                                                const rawLanguage = languageClass.replace('language-', '').toLowerCase();
                                                const normalizedLanguage = languageAliases[rawLanguage] || rawLanguage;

                                                if (normalizedLanguage !== rawLanguage) {
                                                    element.classList.remove(languageClass);
                                                    element.classList.add(`language-${normalizedLanguage}`);
                                                }

                                                if (element.parentElement && element.parentElement.tagName === 'PRE') {
                                                    element.parentElement.dataset.languageLabel = normalizedLanguage.toUpperCase();
                                                }
                                            } else if (element.parentElement && element.parentElement.tagName === 'PRE') {
                                                element.parentElement.dataset.languageLabel = 'TEXT';
                                            }

                                            window.hljs.highlightElement(element);

                                            if (window.hljs.lineNumbersBlock && element.parentElement && element.parentElement.tagName === 'PRE') {
                                                window.hljs.lineNumbersBlock(element);
                                            }

                                            element.dataset.hljsReady = 'true';
                                        });
                                    }

                                    if (window.Splide) {
                                        document.querySelectorAll('[data-slider]').forEach(function(element) {
                                            if (element.dataset.sliderMounted === 'true') {
                                                return;
                                            }

                                            let identifier = element.dataset.slider;

                                            var main = new Splide(`#main-slider-${identifier}`, {
                                                type      : 'fade',
                                                rewind    : true,
                                                pagination: false,
                                                arrows    : false,
                                                drag      : true,
                                              } );

                                              var thumbnails = new Splide(`#thumbnail-slider-${identifier}`, {
                                                fixedWidth  : 96,
                                                fixedHeight : 64,
                                                gap         : 12,
                                                rewind      : true,
                                                pagination  : false,
                                                arrows      : false,
                                                isNavigation: true,
                                                focus       : 'center',
                                                breakpoints : {
                                                  600: {
                                                    fixedWidth : 68,
                                                    fixedHeight: 48,
                                                    gap        : 8,
                                                  },
                                                },
                                              } );

                                              main.sync( thumbnails );
                                              main.mount();
                                              thumbnails.mount();
                                              element.dataset.sliderMounted = 'true';
                                        });
                                    }
                                };

                                if (document.readyState === 'loading') {
                                    document.addEventListener('DOMContentLoaded', initArticleDetail, { once: true });
                                } else {
                                    initArticleDetail();
                                }"
                            </script>
                        }.into_any()
                    })
                }}
            </Suspense>
        </BasicLayout>
    }
}

use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::adapters::driver::leptos_webui::controllers::article_detail_controller;
use crate::adapters::driver::leptos_webui::views::components::blog::ArticleContentRenderer;
use crate::adapters::driver::leptos_webui::views::components::blog::ArticleContentTable;
use crate::adapters::driver::leptos_webui::views::components::blog::ArticleHeader;
use crate::adapters::driver::leptos_webui::views::components::common::SeoMetaTags;
use crate::adapters::driver::leptos_webui::views::components::common::UnexpectedError;
use crate::adapters::driver::leptos_webui::views::layouts::SiteLayout;
use crate::application::domain::article::Article;

#[component]
pub fn BlogDetailPage() -> impl IntoView {
    let route = use_location();
    let page_data = Resource::new(
        move || route.pathname.read().to_string(),
        |slug| article_detail_controller(slug),
    );

    view! {
        <SiteLayout>
            <Suspense fallback=move || { view! { <div>"Loading..."</div> } }>
                {move || {
                    page_data
                    .get_untracked()
                    .map(|data| {
                        if let Err(_) = data {
                            return view! { <UnexpectedError /> }.into_any();
                        }

                        let article: Article = data.unwrap();
                        let table_of_contents_items = article.content_table().clone();

                        view! {
                            <SeoMetaTags metatags=article.metatags().clone() />
                            <link rel="stylesheet" href="/assets/plugins/prismjs/prism-tomorrow.min.css" />
                            <link rel="stylesheet" href="/assets/plugins/prismjs/prism-toolbar.min.css" />
                            {article.has_slider().then(|| {
                                view! {
                                    <link rel="stylesheet" href="/assets/plugins/splidejs/css/splide.min.css" />
                                    <script src="/assets/plugins/splidejs/js/splide.min.js"></script>
                                }
                            })}

                            <div class="pb-12">
                                <div class="article-shell -mx-5 w-auto rounded-none bg-white px-5 py-7 shadow-smoke-shadow transition ease-out duration-[160ms] md:px-8 md:py-10 lg:px-10 lg:py-12 xl:mx-auto xl:w-full xl:rounded-lg xl:px-12">
                                    <div class="flex flex-col lg:grid lg:grid-cols-[minmax(0,1fr)_280px] lg:gap-x-10 xl:grid-cols-[minmax(0,1fr)_300px] xl:gap-x-12">
                                        <div class="lg:col-start-1 lg:row-start-1">
                                            <ArticleHeader article=article.clone() />
                                        </div>
                                        <div class="min-w-0 lg:col-start-1 lg:row-start-2">
                                            <ArticleContentRenderer content=article.content().clone() />
                                        </div>
                                        <div class="lg:col-start-2 lg:row-start-2">
                                            <ArticleContentTable items=table_of_contents_items />
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <script>
                                "window.Prism = window.Prism || {};
                                window.Prism.manual = true;"
                            </script>
                            <script src="/assets/plugins/prismjs/prism-core.min.js"></script>
                            <script src="/assets/plugins/prismjs/prism-toolbar.min.js"></script>
                            <script src="/assets/plugins/prismjs/prism-copy-to-clipboard.min.js"></script>
                            <script src="/assets/plugins/prismjs/prism-autoloader.min.js"></script>

                            <script>
                                "
                                const initArticleDetail = () => {
                                    if (window.__articleTableOfContentsCleanup) {
                                        window.__articleTableOfContentsCleanup();
                                    }

                                    window.__articleTableOfContentsCleanup = null;

                                    if (window.Prism) {
                                        const languageAliases = {
                                            sh: 'bash',
                                            shell: 'bash',
                                            zsh: 'bash',
                                            js: 'javascript',
                                            ts: 'typescript',
                                            py: 'python',
                                            rs: 'rust',
                                            plaintext: 'plain',
                                            text: 'plain',
                                        };

                                        const languageLabels = {
                                            bash: 'Bash',
                                            javascript: 'JavaScript',
                                            typescript: 'TypeScript',
                                            python: 'Python',
                                            rust: 'Rust',
                                            plain: 'Plain text',
                                            markup: 'HTML',
                                            html: 'HTML',
                                            css: 'CSS',
                                            json: 'JSON',
                                            yaml: 'YAML',
                                            toml: 'TOML',
                                            sql: 'SQL',
                                        };

                                        if (window.Prism.plugins && window.Prism.plugins.autoloader) {
                                            window.Prism.plugins.autoloader.languages_path = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/';
                                        }

                                        document.querySelectorAll('pre code, code[class*=\"language-\"]').forEach(function(element) {
                                            if (element.dataset.prismReady === 'true') {
                                                return;
                                            }

                                            element.classList.remove('hljs');
                                            element.removeAttribute('data-hljs-ready');

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
                                                    element.parentElement.dataset.languageLabel = languageLabels[normalizedLanguage] || normalizedLanguage;
                                                }
                                            } else if (element.parentElement && element.parentElement.tagName === 'PRE') {
                                                element.classList.add('language-plain');
                                                element.parentElement.dataset.languageLabel = 'Plain text';
                                            }

                                            window.Prism.highlightElement(element);
                                            element.dataset.prismReady = 'true';
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

                                    const articleContent = document.querySelector('[data-article-content]');
                                    const tableOfContentsItems = Array.from(document.querySelectorAll('[data-article-table-of-contents-index]'));

                                    if (articleContent && tableOfContentsItems.length > 0) {
                                        const articleHeadings = Array.from(articleContent.querySelectorAll('h1, h2, h3'));
                                        const trackedHeadingCount = Math.min(articleHeadings.length, tableOfContentsItems.length);
                                        const trackedArticleHeadings = articleHeadings.slice(0, trackedHeadingCount);

                                        const getCurrentHeadingIndex = () => {
                                            for (let index = trackedArticleHeadings.length - 1; index >= 0; index -= 1) {
                                                if (trackedArticleHeadings[index].getBoundingClientRect().top <= 156) {
                                                    return index;
                                                }
                                            }

                                            return trackedArticleHeadings.length > 0 ? 0 : null;
                                        };

                                        const setActiveTableOfContentsItem = (headingIndex) => {
                                            document.querySelectorAll('[data-article-table-of-contents-index].is-active').forEach(function(item) {
                                                item.classList.remove('is-active');
                                                item.removeAttribute('aria-current');
                                            });

                                            if (typeof headingIndex !== 'number' || Number.isNaN(headingIndex)) {
                                                return;
                                            }

                                            document.querySelectorAll(`[data-article-table-of-contents-index=\"${headingIndex}\"]`).forEach(function(item) {
                                                item.classList.add('is-active');
                                                item.setAttribute('aria-current', 'location');
                                            });
                                        };

                                        setActiveTableOfContentsItem(getCurrentHeadingIndex());

                                        if ('IntersectionObserver' in window && trackedArticleHeadings.length > 0) {
                                            const observer = new IntersectionObserver(function(entries) {
                                                const visibleHeadingIndexes = entries
                                                    .filter(function(entry) {
                                                        return entry.isIntersecting;
                                                    })
                                                    .map(function(entry) {
                                                        return trackedArticleHeadings.indexOf(entry.target);
                                                    })
                                                    .filter(function(headingIndex) {
                                                        return headingIndex >= 0;
                                                    })
                                                    .sort(function(a, b) {
                                                        return a - b;
                                                    });

                                                if (visibleHeadingIndexes.length > 0) {
                                                    setActiveTableOfContentsItem(visibleHeadingIndexes[0]);
                                                    return;
                                                }

                                                setActiveTableOfContentsItem(getCurrentHeadingIndex());
                                            }, {
                                                rootMargin: '-132px 0px -58% 0px',
                                                threshold: [0, 0.2, 0.5, 1],
                                            });

                                            trackedArticleHeadings.forEach(function(heading) {
                                                observer.observe(heading);
                                            });

                                            window.__articleTableOfContentsCleanup = function() {
                                                observer.disconnect();
                                            };
                                        }
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
        </SiteLayout>
    }
}

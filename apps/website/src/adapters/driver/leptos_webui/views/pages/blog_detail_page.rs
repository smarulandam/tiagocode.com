use leptos::prelude::*;
use leptos_meta::{Script, Stylesheet};
use leptos_router::hooks::use_location;

use crate::adapters::driver::leptos_webui::controllers::article_detail_controller;
use crate::adapters::driver::leptos_webui::views::components::blog::{
    ArticleContentRenderer, ArticleHeader,
};
use crate::adapters::driver::leptos_webui::views::components::common::{
    SeoMetaTags, UnexpectedError,
};
use crate::adapters::driver::leptos_webui::views::layouts::SiteLayout;

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

                        let article = data.unwrap();

                        view! {
                            <SeoMetaTags metatags=article.metatags().clone() />
                            <Stylesheet href="/assets/plugins/splidejs/css/splide.min.css" />
                            <Stylesheet href="/assets/plugins/prismjs/prism-tomorrow.min.css" />
                            <Stylesheet href="/assets/plugins/prismjs/prism-toolbar.min.css" />
                            <Script src="/assets/plugins/splidejs/js/splide.min.js" />
                            <script>
                                "window.Prism = window.Prism || {};
                                window.Prism.manual = true;"
                            </script>
                            <Script src="/assets/plugins/prismjs/prism-core.min.js" />
                            <Script src="/assets/plugins/prismjs/prism-toolbar.min.js" />
                            <Script src="/assets/plugins/prismjs/prism-copy-to-clipboard.min.js" />
                            <Script src="/assets/plugins/prismjs/prism-autoloader.min.js" />

                            <div class="pb-12">
                                <div class="article-shell -mx-5 w-auto rounded-none bg-white px-5 py-7 shadow-smoke-shadow transition ease-out duration-[160ms] md:px-8 md:py-10 lg:px-10 lg:py-12 xl:mx-auto xl:w-full xl:rounded-lg xl:px-12">
                                    <ArticleHeader article=article.clone() />
                                    <ArticleContentRenderer content=article.content().clone() />
                                </div>
                            </div>

                            <script>
                                "const initArticleDetail = () => {
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

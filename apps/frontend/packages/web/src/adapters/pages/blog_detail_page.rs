use dioxus::prelude::*;

use ui::components::blog::{ArticleContentRenderer, ArticleHeader};
use ui::components::common::SeoMetaTags;
use ui::{
    PRISM_AUTOLOADER_JS, PRISM_COPY_JS, PRISM_CORE_JS, PRISM_TOMORROW_CSS, PRISM_TOOLBAR_CSS,
    PRISM_TOOLBAR_JS, SPLIDE_CSS, SPLIDE_JS,
};

const PRISM_BOOTSTRAP_SCRIPT: &str = "window.Prism = window.Prism || {}; window.Prism.manual = true;";

const ARTICLE_DETAIL_SCRIPT: &str = r#"
const initArticleDetail = () => {
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

        document.querySelectorAll('pre code, code[class*="language-"]').forEach(function(element) {
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
}
"#;

#[component]
pub fn BlogDetailPage(lang: String, category: String, slug: String) -> Element {
    let full_slug = format!("/{lang}/articles/{category}/{slug}");

    let data = use_loader(move || {
        let full_slug = full_slug.clone();
        async move { api::article_detail_controller(full_slug).await }
    })?;

    let article = data.read().clone();

    rsx! {
        SeoMetaTags { metatags: article.metatags().clone() }
        Stylesheet { href: SPLIDE_CSS }
        Stylesheet { href: PRISM_TOMORROW_CSS }
        Stylesheet { href: PRISM_TOOLBAR_CSS }

        script { src: SPLIDE_JS }
        script { dangerous_inner_html: PRISM_BOOTSTRAP_SCRIPT }
        script { src: PRISM_CORE_JS }
        script { src: PRISM_TOOLBAR_JS }
        script { src: PRISM_COPY_JS }
        script { src: PRISM_AUTOLOADER_JS }

        div { class: "pb-12",
            div {
                class: "article-shell -mx-5 w-auto rounded-none bg-white px-5 py-7 shadow-smoke-shadow transition ease-out duration-[160ms] md:px-8 md:py-10 lg:px-10 lg:py-12 xl:mx-auto xl:w-full xl:rounded-lg xl:px-12",
                ArticleHeader { article: article.clone() }
                ArticleContentRenderer {
                    content: article.content().clone(),
                }
            }
        }

        script { dangerous_inner_html: ARTICLE_DETAIL_SCRIPT }
    }
}

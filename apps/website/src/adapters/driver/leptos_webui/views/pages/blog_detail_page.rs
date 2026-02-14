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

                            <div class="justify-center space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                                <div class="lg:w-3/4 pb-12 article-detail section bg-white px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-smoke-shadow hover:shadow-smoke-shadowHover transition ease-out duration-[160ms]">
                                    <Header article=article.clone() />
                                    <DynamicContent content=article.content().clone() />
                                </div>
                            </div>

                            <script>
                                "document.addEventListener('DOMContentLoaded', (event) => {
                                    if (window.hljs) {
                                        window.hljs.highlightAll();
                                        window.hljs.initLineNumbersOnLoad();
                                    }

                                    if (window.Splide) {
                                        document.querySelectorAll('[data-slider]').forEach(function(element) {
                                        let identifier = element.dataset.slider;

                                        var main = new Splide(`#main-slider-${identifier}`, {
                                            type      : 'fade',
                                            rewind    : true,
                                            pagination: false,
                                            arrows    : false,
                                          } );

                                          var thumbnails = new Splide(`#thumbnail-slider-${identifier}`, {
                                            fixedWidth  : 100,
                                            fixedHeight : 60,
                                            gap         : 10,
                                            rewind      : true,
                                            pagination  : false,
                                            isNavigation: true,
                                            focus: 'center',
                                            breakpoints : {
                                              600: {
                                                fixedWidth : 60,
                                                fixedHeight: 44,
                                              },
                                            },
                                          } );

                                          main.sync( thumbnails );
                                          main.mount();
                                          thumbnails.mount();
                                        });
                                    }
                                });"
                            </script>
                        }.into_any()
                    })
                }}
            </Suspense>
        </BasicLayout>
    }
}

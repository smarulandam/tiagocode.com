use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::adapters::driver::leptos_webui::controllers::articles_list_controller;
use crate::adapters::driver::leptos_webui::views::components::blog::ArticleListSection;
use crate::adapters::driver::leptos_webui::views::components::common::{
    SeoMetaTags, UnexpectedError,
};

#[component]
pub fn BlogListPage() -> impl IntoView {
    let language = use_params_map()
        .get_untracked()
        .get("lang")
        .unwrap_or("en".into());
    let slug = use_params_map()
        .get_untracked()
        .get("page")
        .unwrap_or("articles".into());

    let page_data = Resource::new(
        move || (language.clone(), slug.clone()),
        |(language, slug)| articles_list_controller(language, format!("/{slug}")),
    );

    view! {
        <Suspense fallback=move || { view! { <div class="bg-smoke"></div> } }>
            {move || {
                page_data
                .get()
                .map(|data| {
                    if let Err(_) = data {
                        return view! { <UnexpectedError /> }.into_any();
                    }

                    let (page, categories, articles) = data.unwrap();

                    view! {
                        <SeoMetaTags metatags=page.metatags().clone() />
                        <div class="flex flex-col justify-center gap-6 lg:flex-row lg:gap-8 xl:gap-12">
                            <div class="mb-12 flex w-full flex-col gap-6">
                                <ArticleListSection articles=articles categories=categories />
                            </div>
                        </div>
                    }.into_any()
                })
            }}
        </Suspense>
    }
}

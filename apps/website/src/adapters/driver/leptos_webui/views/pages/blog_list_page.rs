use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::adapters::driver::leptos_webui::controllers::articles_list_controller;
use crate::adapters::driver::leptos_webui::views::components::blog::ListSection;
use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::adapters::driver::leptos_webui::views::layouts::BasicLayout;

#[component]
pub fn BlogListPage() -> impl IntoView {
    let route = use_location();
    let page_data = Resource::new(
        move || route.pathname.read().to_string(),
        |slug| articles_list_controller(slug),
    );

    view! {
        <BasicLayout>
            <Suspense fallback=move || { view! { <div class="bg-smoke"></div> } }>
                {move || {
                    page_data
                    .get_untracked()
                    .map(|data| {
                        if let Err(_) = data {
                            return view! { <UnexpectedError /> }.into_any();
                        }

                        let (page, categories, articles) = data.unwrap();

                        view! {
                            <MetaTags metatags=page.metatags().clone() />
                            <div class="flex flex-col justify-center gap-6 lg:flex-row lg:gap-8 xl:gap-12">
                                <div class="mb-12 flex w-full flex-col gap-6">
                                    <ListSection articles=articles categories=categories />
                                </div>
                            </div>
                        }.into_any()
                    })
                }}
            </Suspense>
        </BasicLayout>
    }
}

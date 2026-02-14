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
            <Suspense fallback=move || { view! { <div class="bg-whitesmoke"></div> } }>
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
                            <div class="justify-center space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                                <div class="w-full space-y-6 mb-12">
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

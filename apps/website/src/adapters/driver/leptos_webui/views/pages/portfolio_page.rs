use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::controllers::portfolio_detail_controller;
use crate::adapters::driver::leptos_webui::views::components::common::{MetaTags, UnexpectedError};
use crate::adapters::driver::leptos_webui::views::components::portfolio::*;
use crate::adapters::driver::leptos_webui::views::layouts::*;

#[component]
pub fn PortfolioPage() -> impl IntoView {
    let page_data = OnceResource::new(portfolio_detail_controller());

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

                        let portfolio = data.unwrap();
                        view! {
                            <MetaTags metatags=portfolio.metatags().clone() />
                            <div class="flex flex-col gap-6 lg:flex-row lg:gap-8 xl:gap-10">
                                <div class="hidden lg:block lg:w-[18.75rem]">
                                    <div class="sidebar-card">
                                        <Sidebar />
                                    </div>
                                </div>
                                <div class="flex flex-1 flex-col gap-6 lg:gap-8">
                                    <DynamicSections sections=portfolio.sections().clone() />
                                </div>
                            </div>
                        }.into_any()
                    })
                }}
            </Suspense>
        </BasicLayout>
    }
}

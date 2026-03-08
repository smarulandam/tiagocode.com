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
                            <div class="flex flex-col justify-center gap-6 lg:flex-row lg:gap-8 xl:gap-12">
                                <div class="lg:w-1/4 hidden lg:block sticky px-4 lg:px-8 xl:px-10 py-3 lg:py-5 xl:py-7 lg:h-fit top-2 lg:top-[80px] bg-teal shadow-smoke-shadow hover:shadow-smoke-shadow-hover rounded-lg">
                                    <Sidebar />
                                </div>
                                <div class="flex flex-col gap-6 lg:w-3/4">
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

use dioxus::prelude::*;

use crate::views::components::common::MetaTagsView;
use crate::views::components::portfolio::{DynamicSections, Sidebar};

#[component]
pub fn PortfolioPageEn() -> Element {
    rsx! { PortfolioPage { lang: "en".to_string() } }
}

#[component]
pub fn PortfolioPageEs() -> Element {
    rsx! { PortfolioPage { lang: "es".to_string() } }
}

#[component]
fn PortfolioPage(lang: String) -> Element {
    let data = use_loader(|| async move { api::portfolio_detail_controller().await })?;
    let portfolio = data.read().clone();

    rsx! {
        MetaTagsView { metatags: portfolio.metatags().clone() }

        div {
            class: "justify-center lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12",
            aside {
                class: "top-2 hidden rounded-lg bg-primary px-4 py-3 shadow-[0_8px_26px_0_rgba(22,24,26,0.15)] transition ease-out duration-[160ms] hover:shadow-[0_10px_30px_0_rgba(22,24,26,0.22)] lg:sticky lg:top-[80px] lg:block lg:h-fit lg:w-1/4 lg:px-8 lg:py-5 xl:px-10 xl:py-7",
                Sidebar {}
            }

            div {
                class: "space-y-6 lg:w-3/4",
                DynamicSections {
                    sections: portfolio.sections().clone(),
                    lang,
                }
            }
        }
    }
}

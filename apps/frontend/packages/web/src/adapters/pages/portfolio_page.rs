use dioxus::prelude::*;

use ui::components::common::SeoMetaTags;
use ui::components::portfolio::{PortfolioSectionRenderer, PortfolioSidebar};

#[component]
pub fn PortfolioPage(lang: String) -> Element {
    let _ = lang;
    let data = use_loader(|| async move { api::portfolio_detail_controller().await })?;
    let portfolio = data.read().clone();

    rsx! {
        SeoMetaTags { metatags: portfolio.metatags().clone() }
        div { class: "flex flex-col justify-center gap-6 lg:flex-row lg:gap-8 xl:gap-12",
            div {
                class: "lg:w-1/4 hidden lg:block sticky px-4 lg:px-8 xl:px-10 py-3 lg:py-5 xl:py-7 lg:h-fit top-2 lg:top-[80px] bg-teal shadow-smoke-shadow hover:shadow-smoke-shadow-hover rounded-lg",
                PortfolioSidebar {}
            }
            div { class: "flex flex-col gap-6 lg:w-3/4",
                PortfolioSectionRenderer {
                    sections: portfolio.sections().clone(),
                }
            }
        }
    }
}

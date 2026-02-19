use dioxus::prelude::*;

use crate::views::components::common::MetaTagsView;
use crate::views::components::portfolio::{DynamicSections, Sidebar};

#[component]
pub fn PortfolioPage(lang: String) -> Element {
    let data = use_loader(|| async move { api::portfolio_detail_controller().await })?;
    let portfolio = data.read().clone();

    rsx! {
        MetaTagsView { metatags: portfolio.metatags().clone() }

        div { class: "items-start justify-center lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-10",
            aside { class: "top-2 hidden rounded-2xl border border-teal-900/35 bg-[linear-gradient(160deg,#0f2a3d_0%,#102535_100%)] px-4 py-3 shadow-[0_12px_24px_-20px_rgba(17,28,42,0.34)] lg:sticky lg:top-[88px] lg:block lg:h-fit lg:w-[16.5rem] lg:px-6 lg:py-5 xl:px-7 xl:py-6",
                Sidebar {}
            }

            div { class: "space-y-6 lg:flex-1",
                DynamicSections { sections: portfolio.sections().clone(), lang }
            }
        }
    }
}

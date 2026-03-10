use dioxus::prelude::*;

use crate::components::common::MetaTags;
use crate::components::portfolio::{DynamicSections, Sidebar};
use content_core::application::domain::portfolio::Portfolio;

#[component]
pub fn PortfolioView(portfolio: Portfolio) -> Element {
    rsx! {
        MetaTags {
            metatags: portfolio.metatags().clone()
        }

        div {
            class: "items-start justify-center lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-10",

            aside {
                class: "top-2 hidden rounded-2xl bg-[linear-gradient(160deg,#0f2a3d_0%,#102535_100%)]",
                class: "px-4 py-3 lg:sticky lg:top-[88px]",
                class: "lg:block lg:h-fit lg:w-[16.5rem] lg:px-6 lg:py-5 xl:px-7 xl:py-6",
                Sidebar {}
            }

            div {
                class: "space-y-6 lg:flex-1",

                DynamicSections {
                    sections: portfolio.sections().clone(),
                }
            }
        }
    }
}

use dioxus::prelude::*;

use crate::components::blog::ListSection;
use crate::components::common::MetaTags;
use content_core::application::domain::article::{Articles, Categories};
use content_core::application::domain::page::Page;

#[component]
pub fn ArticlesListView(
    page: Page,
    categories: Categories,
    articles: Articles,
    selected_category: Option<String>,
) -> Element {
    rsx! {
        MetaTags {
            metatags: page.metatags().clone()
        }

        section {
            class: "w-full pb-12",

            ListSection {
                categories,
                articles,
                selected_category,
            }
        }
    }
}

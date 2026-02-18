use dioxus::prelude::*;

use super::blog_list_page::BlogListContent;

#[component]
pub fn BlogCategoryPage(lang: String, category: String) -> Element {
    rsx! {
        BlogListContent {
            lang,
            category: Some(category),
        }
    }
}

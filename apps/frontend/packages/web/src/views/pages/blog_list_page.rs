use dioxus::prelude::*;

use crate::views::components::blog::ListSection;
use crate::views::components::common::MetaTagsView;

#[component]
pub fn BlogListPage(lang: String) -> Element {
    rsx! {
        BlogListContent { lang, category: None }
    }
}

#[component]
pub fn BlogListContent(lang: String, category: Option<String>) -> Element {
    let slug = if let Some(current_category) = &category {
        format!("/{lang}/articles/{current_category}")
    } else {
        format!("/{lang}/articles")
    };

    let data = use_loader(move || {
        let slug = slug.clone();
        async move { api::articles_list_controller(slug).await }
    })?;

    let (page, categories, articles) = data.read().clone();

    rsx! {
        MetaTagsView { metatags: page.metatags().clone() }

        section { class: "w-full pb-12",
            ListSection {
                categories,
                articles,
                lang,
                selected_category: category,
            }
        }
    }
}

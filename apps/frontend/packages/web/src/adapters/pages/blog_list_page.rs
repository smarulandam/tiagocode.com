use dioxus::prelude::*;

use ui::components::blog::ArticleListSection;
use ui::components::common::SeoMetaTags;

#[component]
pub fn BlogListPage(lang: String) -> Element {
    rsx! {
        BlogListContent { lang, category: None }
    }
}

#[component]
pub fn BlogCategoryPage(lang: String, category: String) -> Element {
    rsx! {
        BlogListContent {
            lang,
            category: Some(category),
        }
    }
}

#[component]
fn BlogListContent(lang: String, category: Option<String>) -> Element {
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
        SeoMetaTags { metatags: page.metatags().clone() }
        div { class: "flex flex-col justify-center gap-6 lg:flex-row lg:gap-8 xl:gap-12",
            div { class: "mb-12 flex w-full flex-col gap-6",
                ArticleListSection { categories, articles }
            }
        }
    }
}

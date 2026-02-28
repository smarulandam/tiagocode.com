use dioxus::prelude::*;

use ui::ArticlesListView;

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
        ArticlesListView {
            page,
            categories,
            articles,
            selected_category: category,
        }
    }
}

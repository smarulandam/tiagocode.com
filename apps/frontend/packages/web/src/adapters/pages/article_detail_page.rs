use dioxus::prelude::*;

use ui::ArticleDetailView;

#[component]
pub fn ArticleDetailPage(lang: String, category: String, slug: String) -> Element {
    let full_slug = format!("/{lang}/articles/{category}/{slug}");

    let data = use_loader(move || {
        let full_slug = full_slug.clone();
        async move { api::article_detail_controller(full_slug).await }
    })?;

    let article = data.read().clone();

    rsx! {
        ArticleDetailView { article }
    }
}

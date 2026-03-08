use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::{Decoration, PrimaryTitle};
use crate::application::domain::article::Article;

#[component]
pub fn Header(article: Article) -> impl IntoView {
    let category = article.category();

    view! {
        <div class="article-hero-meta">
            <div class="article-meta-chip">
                {category.title().to_string()} <span class="ml-2">{category.emoji().to_string()}</span>
            </div>
            <time class="article-date-chip">
                Published at
                <span class="ml-1">{article.created_at().to_string_with_format("%b %d, %Y")}</span>
            </time>
        </div>
        <div class="article-header-group">
            <Decoration text="Article detail".into() />
            <PrimaryTitle text={article.title().to_string()} />
        </div>
    }
}

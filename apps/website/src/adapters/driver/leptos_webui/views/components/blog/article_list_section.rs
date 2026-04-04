use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use super::ArticleCard;
use crate::adapters::driver::leptos_webui::views::components::common::{
    Pill, PrimarySectionTitle, SectionContainer, SectionEyebrow,
};
use crate::application::domain::article::{Article, Category};

#[component]
pub fn ArticleListSection(categories: Vec<Category>, articles: Vec<Article>) -> impl IntoView {
    let are_articles_empty = articles.is_empty();
    let language = use_params_map()
        .get_untracked()
        .get("lang")
        .unwrap_or_else(|| "en".into());

        
    let articles_link = if language == "es" {
        "/es/articulos"
    } else {
        "/articles"
    };

    view! {
        <SectionContainer>
            <div>
                <SectionEyebrow text="My Tech Articles".to_string() />
                <PrimarySectionTitle text="Blog".to_string() />
                <div class="py-6">
                    <Pill link=articles_link.into() text="All".into() />
                    {categories
                        .into_iter()
                        .map(|category| {
                            view! {
                                <Pill
                                    link=category.slug().to_string()
                                    text=category.title().to_string()
                                    emoji=category.emoji().to_string()
                                />
                            }
                        })
                        .collect_view()}
                </div>
            </div>
            <Show
                when=move || !are_articles_empty
                fallback=|| view! {
                    <p class="font-mono font-medium uppercase text-sm text-center tracking-wider relative pt-4 mb-5 text-asparagus">
                        "No articles available. Check back soon!"
                    </p>
                }
            >
                <div class="mt-6 grid grid-cols-1 gap-x-6 gap-y-8 lg:mt-8 lg:grid-cols-2 xl:grid-cols-3">
                    {articles
                        .clone()
                        .into_iter()
                        .map(|article| {
                            view! {
                                <ArticleCard
                                    date=article.created_at().to_string_with_format("%b %d, %Y")
                                    title=article.title().to_string()
                                    summary=article.summary().to_string()
                                    slug=article.slug().to_string()
                                    category=article.category().clone()
                                    thumbnail=article.thumbnail().clone()
                                />
                            }
                        })
                        .collect_view()}
                </div>
            </Show>
        </SectionContainer>
    }
}

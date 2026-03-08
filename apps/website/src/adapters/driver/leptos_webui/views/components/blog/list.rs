use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::application::domain::article::{Article, Category};
use crate::application::domain::common::Image;

#[component]
pub fn ListSection(categories: Vec<Category>, articles: Vec<Article>) -> impl IntoView {
    let are_articles_empty = articles.is_empty().clone();

    view! {
        <Container>
            <div class="section-heading">
                <Decoration text="My Tech Articles".to_string() />
                <PrimaryTitle text="Blog".to_string() />
            </div>

            <div class="filters-row">
                <Pill link="/en/articles".into() text="All".into() />
                {categories
                    .into_iter()
                    .map(|c| {
                        view! {
                            <Pill
                                link=c.slug().to_string()
                                text=c.title().to_string()
                                emoji=c.emoji().to_string()
                            />
                        }
                    })
                    .collect_view()}
            </div>

            <Show
                when=move || !are_articles_empty
                fallback=|| view! {
                    <p class="empty-state">
                        "No articles available. Check back soon!"
                    </p>
                }
            >
                <div class="mt-8 grid grid-cols-1 gap-5 xl:grid-cols-2">
                    {articles
                        .clone()
                        .into_iter()
                        .map(|a| {
                            view! {
                                <ArticleCard
                                    date=a.created_at().to_string_with_format("%b %d, %Y")
                                    title=a.title().to_string()
                                    summary=a.summary().to_string()
                                    slug=a.slug().to_string()
                                    category=a.category().clone()
                                    thumbnail=a.thumbnail().clone()
                                />
                            }
                        })
                        .collect_view()}
                </div>
            </Show>
        </Container>
    }
}

#[component]
pub fn ArticleCard(
    date: String,
    slug: String,
    title: String,
    summary: String,
    thumbnail: Image,
    category: Category,
) -> impl IntoView {
    view! {
        <article class="article-list-card">
            <div class="article-list-card__media">
                <Img image=thumbnail class="h-full w-full object-cover" />
                <div class="article-list-card__category">
                    <a href=category.slug().to_string() target="_self">
                        {category.title().to_string()}
                        <span class="ml-2">{category.emoji().to_string()}</span>
                    </a>
                </div>
            </div>

            <div class="article-list-card__body">
                <span class="article-list-card__date">{date}</span>
                <h2 class="article-list-card__title">{title}</h2>
                <p class="article-list-card__summary">
                    {if summary.chars().count() > 110 {
                        summary.chars().take(110).collect::<String>() + "..."
                    } else {
                        summary
                    }}
                </p>
                <div class="pt-1">
                    <a href=slug target="_self" class="button-primary">
                        Read More
                    </a>
                </div>
            </div>
        </article>
    }
}

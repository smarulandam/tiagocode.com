use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::application::domain::article::Article;
use crate::application::domain::common::Image;

#[component]
pub fn BlogSection(
    title: String,
    subtitle: String,
    text: String,
    articles: Vec<Article>,
) -> impl IntoView {
    let are_articles_empty = articles.is_empty().clone();

    view! {
        <Container id="blog".into()>
            <div class="section-heading">
                <Decoration text=subtitle />
                <SecondaryTitle text=title />
                <Description text=text />
            </div>
            <div class="mt-8 flex flex-col gap-5 lg:mt-10">
                <Show
                    when=move || !are_articles_empty
                    fallback=|| view! {
                        <p class="empty-state">
                            "No articles available. Check back soon!"
                        </p>
                    }>
                    {articles
                        .clone()
                        .into_iter()
                        .map(|a| {
                            view! {
                                <FeaturedArticleCard
                                    published_at=a.created_at().to_string_with_format("%b %d, %Y")
                                    title=a.title().to_string()
                                    summary=a.summary().to_string()
                                    link=a.slug().to_string()
                                    category=format!("{} {}", a.category().title().to_string(), a.category().emoji().to_string())
                                    thumbnail=a.thumbnail().clone()
                                />
                            }
                        }).collect_view()
                    }
                </Show>
            </div>
        </Container>
    }
}

#[component]
pub fn FeaturedArticleCard(
    published_at: String,
    title: String,
    summary: String,
    link: String,
    thumbnail: Image,
    category: String,
) -> impl IntoView {
    view! {
        <article class="article-list-card article-list-card--featured">
            <div class="article-list-card__media">
                <Img image=thumbnail class="h-full w-full object-cover" />
                <div class="article-list-card__category article-list-card__category--top">
                    {category}
                </div>
            </div>
            <div class="article-list-card__body">
                <span class="article-list-card__date">{format!("Posted on {}", published_at)}</span>
                <h3 class="article-list-card__title">{title}</h3>
                <p class="article-list-card__summary">{summary}</p>
                <div class="pt-1">
                    <a href=link target="_self" class="button-secondary">
                        Read More
                    </a>
                </div>
            </div>
        </article>
    }
}

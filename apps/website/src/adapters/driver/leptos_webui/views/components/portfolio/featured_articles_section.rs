use leptos::prelude::*;
use leptos_router::components::A;

use super::FeaturedArticleCard;
use crate::adapters::driver::leptos_webui::views::components::common::{
    SectionContainer, SectionDescription, SectionEyebrow, SectionTitle,
};
use crate::application::domain::article::Article;

#[component]
pub fn FeaturedArticlesSection(
    title: String,
    subtitle: String,
    text: String,
    articles: Vec<Article>,
) -> impl IntoView {
    let are_articles_empty = articles.is_empty();

    view! {
        <SectionContainer id="blog".into()>
            <div class="">
                <SectionEyebrow text=subtitle />
                <SectionTitle text=title />
                <SectionDescription text=text />
            </div>
            <div class="mt-6 flex flex-col gap-6 lg:mt-10">
                <Show
                    when=move || !are_articles_empty
                    fallback=|| view! {
                        <p class="font-mono font-medium uppercase text-sm text-center tracking-wider relative pt-4 mb-5 text-asparagus">
                            "No articles available. Check back soon!"
                        </p>
                    }
                >
                    <>
                        {articles
                            .clone()
                            .into_iter()
                            .map(|article| {
                                view! {
                                    <FeaturedArticleCard
                                        published_at=article.created_at().to_string_with_format("%b %d, %Y")
                                        title=article.title().to_string()
                                        summary=article.summary().to_string()
                                        link=article.slug().to_string()
                                        category=format!("{} {}", article.category().title().to_string(), article.category().emoji().to_string())
                                        thumbnail=article.thumbnail().clone()
                                    />
                                }
                            })
                            .collect_view()}
                        <div class="flex justify-center pt-6">
                            <A
                                href="/en/articles"
                                attr:aria-label="View all articles"
                                attr:class="inline-flex items-center justify-center border-b border-teal/35 px-1 pb-1 pt-4 text-base font-medium text-teal transition duration-[120ms] ease-out hover:border-teal hover:text-deepsea focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal/25 focus-visible:ring-offset-2"
                            >
                                "View All Articles"
                            </A>
                        </div>
                    </>
                </Show>
            </div>
        </SectionContainer>
    }
}

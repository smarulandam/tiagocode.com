use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::application::domain::article::{Article, Category};
use crate::application::domain::common::Image;

#[component]
pub fn ListSection(categories: Vec<Category>, articles: Vec<Article>) -> impl IntoView {
    let are_articles_empty = articles.is_empty().clone();

    view! {
        <Container>
            <div>
                <Decoration text="My Tech Articles".to_string() />
                <PrimaryTitle text="Blog".to_string() />
                <div class="py-6">
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
                        }).collect_view()
                    }
                </div>
            </div>
            <Show
                when=move || !are_articles_empty
                fallback=|| view! {
                    <p class="font-mono font-medium uppercase text-sm text-center tracking-wider relative pt-4 mb-5 text-asparagus">
                        "No articles available. Check back soon!"
                    </p>
                }>
                <div class="grid grid-cols-1 xl:grid-cols-2 gap-4 mt-6 lg:mt-3">
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
                        }).collect_view()
                    }
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
        <article class="md:flex md:items-start md:justify-center mt-8">
            <div class="overflow-hidden relative rounded-lg group  flex-shrink-0">
                <Img image=thumbnail class="w-full transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.5px]" />
                <div class="absolute bottom-0 left-0 right-0 rounded-none text-center bg-black/50 px-4 py-3 text-white backdrop-blur-[5px] font-mono font-bold uppercase text-sm tracking-[0.5px]">
                    <a href=category.slug().to_string() target="_self">
                        {category.title().to_string()}
                        <span class="ml-2">{category.emoji().to_string()}</span>
                    </a>
                </div>
            </div>
            <div class="md:pl-7 md:mt-0  flex-grow">
                <span class="text-zeus dark:text-white/70">{date}</span>
                <h2 class="font-poppins font-semibold text-lg mt-2">{title}</h2>
                <p class="text-zeus dark:text-white/70">{
                    if summary.chars().count() > 110 {
                        summary.chars().take(110).collect::<String>() + "..."
                    } else {
                        summary
                    }
                }</p>
                <a href=slug target="_self" class="inline-block text-white hover:text-zeus bg-black hover:bg-white hover:border hover:border-black border-dashed rounded-full px-6 py-3 mt-3 lg:mt-4 font-mono text-sm transition ease-out duration-[120ms]">
                    Read More
                </a>
            </div>
        </article>
    }
}

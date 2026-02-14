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
            <div class="">
                <Decoration text=subtitle />
                <SecondaryTitle text=title />
                <Description text=text />
            </div>
            <div class="mt-6 lg:mt-12 space-y-8 md:space-y-6">
                <Show
                    when=move || !are_articles_empty
                    fallback=|| view! {
                        <p class="font-mono font-medium uppercase text-sm text-center tracking-wider relative pt-4 mb-5 text-asparagus">
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
        <article class="sm:flex md:items-start">
            <div class="overflow-hidden relative rounded-lg group flex-shrink-0">
                <Img image=thumbnail class="w-full sm:max-w-[340px] transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.5px]" />
                <div class="absolute top-4 left-4 bg-black/20 px-4 py-2 rounded-full text-white backdrop-blur-[5px] font-mono font-normal uppercase text-sm tracking-[0.5px]">
                    {category}
                </div>
            </div>
            <div class="mt-5 md:pl-7 md:mt-0 flex-grow">
                <span class="text-zeus dark:text-white/70">{format!("Posted on {}", published_at)}</span>
                <h3 class="font-poppins font-semibold text-2xl mt-2">{title}</h3>
                <p class="text-zeus dark:text-white/70">{summary}</p>
                <a href=link target="_self"  class="inline-block border border-black border-dashed rounded-full px-6 py-3 mt-3 lg:mt-4 font-mono text-sm hover:bg-black hover:text-white transition ease-out duration-[120ms] dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black">
                    Read More
                </a>
            </div>
        </article>
    }
}

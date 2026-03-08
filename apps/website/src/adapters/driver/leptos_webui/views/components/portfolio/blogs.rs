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
            <div class="mt-6 flex flex-col gap-6 lg:mt-10">
                <Show
                    when=move || !are_articles_empty
                    fallback=|| view! {
                        <p class="font-mono font-medium uppercase text-sm text-center tracking-wider relative pt-4 mb-5 text-asparagus">
                            "No articles available. Check back soon!"
                        </p>
                    }>
                    <>
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
                        <div class="flex justify-center pt-6">
                            <a
                                href="/en/articles"
                                target="_self"
                                aria-label="View all articles"
                                class="inline-flex items-center justify-center border-b border-teal/35 px-1 pb-1 pt-4 text-base font-medium text-teal transition duration-[120ms] ease-out hover:border-teal hover:text-deepsea focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal/25 focus-visible:ring-offset-2"
                            >
                                "View All Articles"
                            </a>
                        </div>
                    </>
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
    let cta_label = format!("Read more: {}", title.clone());

    view! {
        <article class="group rounded-[1.05rem]  transition duration-[160ms] ease-out hover:border-teal/20 sm:flex sm:items-start sm:gap-5">
            <div class="relative overflow-hidden rounded-[0.9rem] bg-smoke flex-shrink-0">
                <Img image=thumbnail class="w-full sm:w-[300px] lg:w-[320px] transition ease-custom duration-500 group-hover:scale-[1.02] group-hover:saturate-[1.03]" />
            </div>
            <div class="mt-4 flex min-w-0 flex-1 flex-col justify-center sm:mt-0">
                <span class="text-base font-medium leading-6 text-zeus/60">
                    {published_at}
                </span>
                <h3 class="mt-2 text-[1.45rem] font-poppins font-semibold leading-[1.12] text-deepsea md:text-[1.55rem]">
                    <a href=link.clone() target="_self" class="transition duration-150 ease-out group-hover:text-teal">
                        {title}
                    </a>
                </h3>
                <div class="mt-3">
                    <span class="inline-flex items-center rounded-full border border-teal/15 bg-teal/8 px-3.5 py-1.5 font-medium text-base leading-none text-teal">
                        {category}
                    </span>
                </div>
                <p class="mt-2.5 line-clamp-3 text-base leading-7 text-zeus/72 md:line-clamp-2">
                    {summary}
                </p>
                <a
                    href=link
                    target="_self"
                    aria-label=cta_label
                    class="mt-3 inline-block self-start rounded-full border border-black border-dashed px-6 py-3 font-mono text-sm transition duration-[120ms] ease-out hover:bg-black hover:text-white dark:border-white dark:text-white dark:hover:bg-white dark:hover:text-black lg:mt-4"
                >
                    "Read More"
                </a>
            </div>
        </article>
    }
}

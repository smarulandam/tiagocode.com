use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::ImageView;
use crate::application::domain::common::Image;

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
            <div class="relative aspect-[26/21] w-full overflow-hidden rounded-[0.9rem] bg-smoke flex-shrink-0 sm:w-[300px] lg:w-[320px]">
                <ImageView
                    image=thumbnail
                    with_wrapper=false
                    class="h-full w-full object-cover transition ease-custom duration-500 group-hover:scale-[1.02] group-hover:saturate-[1.03]"
                />
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

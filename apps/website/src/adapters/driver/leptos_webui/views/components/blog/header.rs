use leptos::prelude::*;

use crate::application::domain::article::Article;

#[component]
pub fn Header(article: Article) -> impl IntoView {
    let category = article.category();

    view! {
        <div class="mx-auto w-full max-w-[760px]">
            <div class="flex flex-wrap items-center gap-3">
                <a
                    href=category.slug().to_string()
                    target="_self"
                    class="inline-flex items-center rounded-full border border-teal/15 bg-teal/8 px-3.5 py-1.5 text-base font-medium leading-none text-teal transition duration-[120ms] ease-out hover:border-teal/25 hover:bg-teal/12"
                >
                    {category.title().to_string()}
                    <span class="ml-2">{category.emoji().to_string()}</span>
                </a>
                <time class="text-base font-medium leading-6 text-zeus/58">
                    {article.created_at().to_string_with_format("%b %d, %Y")}
                </time>
            </div>

            <h1 class="mt-4 font-poppins text-[2.35rem] font-semibold leading-[1.04] text-deepsea md:text-[3.1rem]">
                {article.title().to_string()}
            </h1>

            <p class="mt-5 max-w-[65ch] font-opensans text-base leading-8 text-zeus/78 md:text-[1.08rem]">
                {article.summary().to_string()}
            </p>

            <div class="mt-8 flex items-center gap-4">
                <img
                    src="/assets/images/author.png"
                    alt="Santiago Marulanda"
                    class="h-14 w-14 rounded-full object-cover ring-2 ring-teal/10"
                />
                <div>
                    <p class="font-poppins text-lg font-semibold leading-none text-deepsea">
                        "Santiago Marulanda"
                    </p>
                    <p class="mt-1 text-sm font-medium text-zeus/56">
                        "Author · Tiagocode"
                    </p>
                </div>
            </div>

            <div class="mt-8 border-b border-black/8"></div>
        </div>
    }
}

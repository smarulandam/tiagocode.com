use dioxus::prelude::*;

use content_core::application::domain::article::Article;

#[component]
pub fn ArticleHeader(article: Article) -> Element {
    let category = article.category().clone();

    rsx! {
        div { class: "mx-auto w-full max-w-[760px]",
            h1 {
                class: "font-poppins text-[2.35rem] font-semibold leading-[1.04] text-deepsea md:text-[3.1rem]",
                "{article.title()}"
            }

            p {
                class: "mt-5 max-w-[65ch] font-opensans text-base leading-8 text-zeus/78 md:text-[1.08rem]",
                "{article.summary()}"
            }

            div { class: "mt-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between",
                div { class: "flex items-center gap-4",
                    img {
                        src: asset!("/assets/images/author.png"),
                        alt: "Santiago Marulanda",
                        class: "h-14 w-14 rounded-full object-cover ring-2 ring-teal/10",
                    }
                    div {
                        p {
                            class: "font-poppins text-lg font-semibold leading-none text-deepsea",
                            "Santiago Marulanda"
                        }
                        p { class: "mt-1 text-sm font-medium text-zeus/56", "Author · Tiagocode" }
                    }
                }

                div { class: "flex flex-col items-start gap-2 sm:items-end",
                    a {
                        href: category.slug().to_string(),
                        target: "_self",
                        class: "inline-flex items-center rounded-full border border-teal/15 bg-teal/8 px-3.5 py-1.5 text-base font-medium leading-none text-teal transition duration-[120ms] ease-out hover:border-teal/25 hover:bg-teal/12",
                        "{category.title()}"
                        span { class: "ml-2", "{category.emoji()}" }
                    }
                    time { class: "text-base font-medium leading-6 text-teal/82 sm:text-right",
                        "Published on · "
                        span { "{article.created_at().to_string_with_format(\"%b %d, %Y\")}" }
                    }
                }
            }

            div { class: "mt-8 border-b border-black/8" }
        }
    }
}

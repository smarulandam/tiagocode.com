use dioxus::prelude::*;

use crate::components::common::ImageView;
use content_core::application::domain::article::Category;
use content_core::application::domain::common::Image;

#[component]
pub fn ArticleCard(
    date: String,
    slug: String,
    title: String,
    summary: String,
    thumbnail: Image,
    category: Category,
) -> Element {
    let cta_label = format!("Read article: {}", title);
    let category_href = category.slug().to_string();
    let category_label = format!(
        "{} {}",
        category.title().to_string(),
        category.emoji().to_string()
    );

    rsx! {
        article { class: "group flex h-full flex-col",
            div { class: "relative aspect-[16/10] overflow-hidden rounded-[0.9rem] bg-smoke",
                ImageView {
                    image: thumbnail,
                    class: "h-full w-full object-cover transition ease-custom duration-500 group-hover:scale-[1.02] group-hover:saturate-[1.03]"
                        .to_string(),
                }
            }
            div { class: "mt-4 flex min-w-0 flex-1 flex-col",
                span { class: "text-base font-medium leading-6 text-zeus/60", "{date}" }
                h2 { class: "mt-2 line-clamp-2 text-[1.3rem] font-poppins font-semibold leading-[1.14] text-deepsea lg:text-[1.4rem]",
                    a {
                        href: slug.clone(),
                        target: "_self",
                        class: "transition duration-150 ease-out group-hover:text-teal",
                        "{title}"
                    }
                }
                div { class: "mt-3",
                    a {
                        href: category_href,
                        target: "_self",
                        class: "inline-flex items-center rounded-full border border-teal/15 bg-teal/8 px-3.5 py-1.5 font-medium text-base leading-none text-teal transition duration-150 ease-out hover:border-teal/25 hover:bg-teal/12",
                        "{category_label}"
                    }
                }
                p { class: "mt-2.5 line-clamp-3 text-base leading-7 text-zeus/72",
                    "{summary}"
                }
                div { class: "mt-auto pt-4",
                    a {
                        href: slug,
                        target: "_self",
                        aria_label: cta_label,
                        class: "inline-block self-start rounded-full border border-black border-dashed px-6 py-3 font-mono text-sm transition duration-[120ms] ease-out hover:bg-black hover:text-white dark:border-white dark:text-white dark:hover:bg-white dark:hover:text-black",
                        "Read article"
                    }
                }
            }
        }
    }
}

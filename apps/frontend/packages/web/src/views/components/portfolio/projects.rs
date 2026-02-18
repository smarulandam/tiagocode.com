use dioxus::prelude::*;

use content_core::application::domain::common::{Image, Link};
use content_core::application::domain::portfolio::Projects;

use crate::views::components::common::Img;

#[component]
pub fn ProjectsSection(data: Projects) -> Element {
    rsx! {
        section {
            id: "portfolio",
            class: "section rounded-lg bg-white px-6 py-8 shadow-[0_8px_26px_0_rgba(22,24,26,0.15)] transition ease-out duration-[160ms] hover:shadow-[0_10px_30px_0_rgba(22,24,26,0.22)] md:px-8 md:py-10 lg:p-12",

            div {
                p {
                    class: "relative mb-5 pt-4 font-mono text-sm font-medium uppercase tracking-wider text-accent before:pr-2 before:content-['//']",
                    "{data.subtitle()}"
                }
                h2 { class: "mb-2 text-4xl font-display font-semibold text-primary", "{data.title()}" }
                p { class: "text-zeus", "{data.text()}" }
            }

            div {
                class: "mt-6 lg:mt-12",
                div {
                    class: "portfolio-grid mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3",
                    for project in data.projects().iter() {
                        FeaturedProjectCard {
                            title: project.title().to_string(),
                            link: project.link().clone(),
                            image: project.image().clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FeaturedProjectCard(title: String, link: Link, image: Image) -> Element {
    rsx! {
        div { class: "portfolio-item category-1",
            div {
                class: "category-1 group relative overflow-hidden rounded-lg after:absolute after:top-0 after:left-0 after:h-full after:w-full after:bg-gradient-to-t after:from-black/30 after:to-transparent after:opacity-0 after:transition after:duration-[160ms] after:ease-out after:content-[''] hover:after:opacity-100",
                Img {
                    image: image,
                    class: Some("transition duration-500 ease-out group-hover:scale-105 group-hover:blur-[1.4px]".to_string()),
                }
                div {
                    class: "invisible absolute bottom-0 left-0 z-[1] w-full translate-y-2 px-2 pb-6 text-center opacity-0 transition duration-[160ms] ease-out group-hover:visible group-hover:translate-y-0 group-hover:mb-0 group-hover:opacity-100",
                    a {
                        href: link.url().to_string(),
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "font-display text-3xl font-semibold tracking-[0.5px] text-white transition-all duration-100 ease-linear lg:text-4xl",
                        "{title}"
                    }
                }
            }
        }
    }
}

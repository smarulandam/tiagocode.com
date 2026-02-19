use dioxus::prelude::*;

use content_core::application::domain::common::{Image, Link};
use content_core::application::domain::portfolio::Projects;

use crate::views::components::common::{Container, Decoration, Description, Img, SecondaryTitle};

#[component]
pub fn ProjectsSection(data: Projects) -> Element {
    rsx! {
        Container { id: Some("portfolio".to_string()),

            div {
                Decoration { text: data.subtitle().to_string() }
                SecondaryTitle { text: data.title().to_string() }
                Description { text: data.text().to_string() }
            }

            div { class: "mt-6 lg:mt-12",
                div { class: "portfolio-grid mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3",
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
        div { class: "rounded-2xl transition duration-200 ease-out hover:-translate-y-[2px]",
            div { class: "group relative overflow-hidden rounded-xl border border-[#d6dee8] bg-white shadow-[0_12px_24px_-20px_rgba(17,28,42,0.24)] transition duration-200 ease-out hover:border-accent/30 hover:shadow-[0_20px_34px_-24px_rgba(17,28,42,0.3)] after:absolute after:top-0 after:left-0 after:h-full after:w-full after:bg-gradient-to-t after:from-black/55 after:to-transparent after:opacity-70 after:transition after:duration-[180ms] after:ease-out after:content-['']",
                Img {
                    image,
                    class: Some(
                        "transition duration-500 ease-out group-hover:scale-105"
                            .to_string(),
                    ),
                }
                div { class: "absolute bottom-0 left-0 z-[1] w-full px-4 pb-4 text-left",
                    a {
                        href: link.url().to_string(),
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "font-display text-xl font-semibold tracking-tight text-white drop-shadow-[0_8px_14px_rgba(3,8,18,0.58)] transition-all duration-100 ease-linear lg:text-2xl",
                        "{title}"
                    }
                }
            }
        }
    }
}

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
        div { class: "portfolio-item category-1",
            div { class: "category-1 group relative overflow-hidden rounded-lg after:absolute after:top-0 after:left-0 after:h-full after:w-full after:bg-gradient-to-t after:from-black/30 after:to-transparent after:opacity-0 after:transition after:duration-[160ms] after:ease-out after:content-[''] hover:after:opacity-100",
                Img {
                    image,
                    class: Some(
                        "transition duration-500 ease-out group-hover:scale-105 group-hover:blur-[1.4px]"
                            .to_string(),
                    ),
                }
                div { class: "invisible absolute bottom-0 left-0 z-[1] w-full translate-y-2 px-2 pb-6 text-center opacity-0 transition duration-[160ms] ease-out group-hover:visible group-hover:translate-y-0 group-hover:mb-0 group-hover:opacity-100",
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

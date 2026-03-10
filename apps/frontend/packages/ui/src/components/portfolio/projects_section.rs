use dioxus::prelude::*;

use super::FeaturedProjectCard;
use crate::components::common::{
    SectionContainer, SectionDescription, SectionEyebrow, SectionTitle,
};
use content_core::application::domain::common::Project;

#[component]
pub fn ProjectsSection(
    title: String,
    subtitle: String,
    text: String,
    projects: Vec<Project>,
) -> Element {
    rsx! {
        SectionContainer { id: "portfolio".to_string(),
            div {
                SectionEyebrow { text: subtitle }
                SectionTitle { text: title }
                SectionDescription { text }
            }
            div { class: "mt-6 lg:mt-12",
                div { class: "portfolio-grid grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-6",
                    for project in projects {
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

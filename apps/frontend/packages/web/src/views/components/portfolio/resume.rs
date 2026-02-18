use dioxus::prelude::*;

use content_core::application::domain::portfolio::Resume;

use crate::views::components::common::{Container, Decoration, Description, SecondaryTitle};
use crate::views::components::portfolio::{TimelineIcon, TimelineSection, TimelineSectionItem};

#[component]
pub fn ResumeSection(data: Resume) -> Element {
    rsx! {
        Container { id: Some("resume".to_string()),

            div {
                Decoration { text: data.subtitle().to_string() }
                SecondaryTitle { text: data.title().to_string() }
                Description { text: data.text().to_string() }
            }

            div { class: "mt-6 grid grid-cols-1 gap-6 md:gap-8 lg:mt-12 lg:gap-10 md:grid-cols-2",
                TimelineSection { icon: TimelineIcon::Education,
                    for item in data.education().items().iter() {
                        TimelineSectionItem {
                            date: item.date().to_string(),
                            title: item.title().to_string(),
                            subtitle: item.subtitle().to_string(),
                        }
                    }
                }

                TimelineSection { icon: TimelineIcon::Experience,
                    for item in data.experience().items().iter() {
                        TimelineSectionItem {
                            date: item.date().to_string(),
                            title: item.title().to_string(),
                            subtitle: item.subtitle().to_string(),
                        }
                    }
                }
            }
        }
    }
}

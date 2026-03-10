use dioxus::prelude::*;

use super::{TimelineEntry, TimelineGroup, TimelineIcon};
use crate::components::common::{
    SectionContainer, SectionDescription, SectionEyebrow, SectionTitle,
};
use content_core::application::domain::common::Timeline;

#[component]
pub fn ResumeSection(
    title: String,
    subtitle: String,
    text: String,
    education: Timeline,
    experience: Timeline,
) -> Element {
    rsx! {
        SectionContainer { id: "resume".to_string(),
            div {
                SectionEyebrow { text: subtitle }
                SectionTitle { text: title }
                SectionDescription { text }
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 md:gap-8 lg:gap-10 mt-6 lg:mt-12",
                TimelineGroup { icon: TimelineIcon::Education,
                    for education_item in education.items() {
                        TimelineEntry {
                            date: education_item.date().to_string(),
                            title: education_item.title().to_string(),
                            subtitle: education_item.subtitle().to_string(),
                        }
                    }
                }
                TimelineGroup { icon: TimelineIcon::Experience,
                    for experience_item in experience.items() {
                        TimelineEntry {
                            date: experience_item.date().to_string(),
                            title: experience_item.title().to_string(),
                            subtitle: experience_item.subtitle().to_string(),
                        }
                    }
                }
            }
        }
    }
}

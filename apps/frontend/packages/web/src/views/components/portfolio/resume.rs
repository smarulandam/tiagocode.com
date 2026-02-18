use dioxus::prelude::*;

use content_core::application::domain::portfolio::Resume;

use crate::views::components::portfolio::{TimelineSection, TimelineSectionItem};

#[component]
pub fn ResumeSection(data: Resume) -> Element {
    rsx! {
        section {
            id: "resume",
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
                class: "mt-6 grid grid-cols-1 gap-6 md:gap-8 lg:mt-12 lg:gap-10 md:grid-cols-2",
                TimelineSection { icon_class: "bi bi-mortarboard",
                    for item in data.education().items().iter() {
                        TimelineSectionItem {
                            date: item.date().to_string(),
                            title: item.title().to_string(),
                            subtitle: item.subtitle().to_string(),
                        }
                    }
                }

                TimelineSection { icon_class: "bi bi-briefcase",
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

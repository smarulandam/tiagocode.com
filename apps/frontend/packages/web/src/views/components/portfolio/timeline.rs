use dioxus::prelude::*;

use crate::views::components::common::{EducationIcon, ExperienceIcon};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimelineIcon {
    Education,
    Experience,
}

#[component]
pub fn TimelineSection(icon: TimelineIcon, children: Element) -> Element {
    rsx! {
        div { class: "timeline-section relative space-y-7 pl-7",
            div { class: "timeline-icon text-3xl leading-none",
                match icon {
                    TimelineIcon::Education => rsx! {
                        EducationIcon { class: Some("h-6 w-6".to_string()) }
                    },
                    TimelineIcon::Experience => rsx! {
                        ExperienceIcon { class: Some("h-6 w-6".to_string()) }
                    },
                }
            }
            {children}
        }
    }
}

#[component]
pub fn TimelineSectionItem(date: String, title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "group",
            div { class: "timeline-date relative inline-block rounded-full border border-border bg-muted px-4 py-2 font-mono text-xs font-medium uppercase tracking-wider text-muted-foreground transition duration-100 ease-linear group-hover:text-foreground",
                "{date}"
            }
            h3 { class: "mb-1 mt-2 font-display text-lg font-semibold text-primary lg:mb-2 lg:mt-3 lg:text-xl",
                "{title}"
            }
            span { class: "text-zeus/90", "{subtitle}" }
        }
    }
}

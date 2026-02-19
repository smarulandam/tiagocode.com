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
        div { class: "relative space-y-7 pl-7 before:absolute before:left-[0.6rem] before:top-[0.6rem] before:h-[calc(100%-0.8rem)] before:w-px before:bg-border/85 before:content-['']",
            div { class: "relative z-[1] inline-flex h-[1.9rem] w-[1.9rem] items-center justify-center rounded-[0.65rem] border border-border/90 bg-surface-soft/80 text-primary shadow-[0_8px_14px_-12px_rgba(15,24,36,0.6)]",
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
            div { class: "relative inline-block rounded-full border border-[#d6dee8] bg-[#f8fbfe] px-4 py-2 font-mono text-xs font-medium uppercase tracking-wider text-muted-foreground transition duration-100 ease-linear group-hover:text-foreground before:absolute before:left-[-1.15rem] before:top-1/2 before:h-px before:w-[0.85rem] before:-translate-y-1/2 before:bg-border/90 before:content-[''] after:absolute after:left-[-1.42rem] after:top-1/2 after:h-[0.42rem] after:w-[0.42rem] after:-translate-y-1/2 after:rounded-full after:bg-primary/85 after:content-['']",
                "{date}"
            }
            h3 { class: "mb-1 mt-2 font-display text-lg font-semibold text-primary lg:mb-2 lg:mt-3 lg:text-xl",
                "{title}"
            }
            span { class: "text-zeus/90", "{subtitle}" }
        }
    }
}

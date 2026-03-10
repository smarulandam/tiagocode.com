use dioxus::prelude::*;

use crate::components::common::{
    ImageView, Pill, PrimarySectionTitle, SectionContainer, SectionDescription, SectionEyebrow,
};
use content_core::application::domain::common::{Document, Image};

#[component]
pub fn AboutMeSection(
    subtitle: String,
    title: String,
    text: String,
    skills: Vec<String>,
    profile_picture: Image,
    years_of_experience: u8,
    cv_document: Document,
) -> Element {
    let _ = cv_document;

    rsx! {
        SectionContainer { id: "about".to_string(),
            div { class: "flex flex-col gap-5 lg:flex-row lg:gap-10",
                div {
                    div { class: "flex justify-center relative h-fit",
                        ImageView {
                            image: profile_picture,
                            class: "min-w-52 min-h-52 max-w-64 max-h-64 rounded-full".to_string(),
                        }
                    }
                    div { class: "flex items-center justify-center",
                        div { class: "pe-2",
                            div { class: "font-mono font-semibold text-6xl stroke-text",
                                span { class: "counter", "{years_of_experience}" }
                            }
                        }
                        div { class: "py-2",
                            span { class: "block text-2xl font-normal mb-1", "+" }
                            p {
                                class: "font-mono font-medium text-sm uppercase tracking-[0.5px]",
                                "Years of Experience"
                            }
                        }
                    }
                }
                div {
                    SectionEyebrow { text: subtitle }
                    PrimarySectionTitle { text: title }
                    SectionDescription { text }
                    div { class: "mt-5 flex flex-wrap gap-3",
                        for skill in skills {
                            Pill {
                                text: skill,
                                class: "me-0".to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}

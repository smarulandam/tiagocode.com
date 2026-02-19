use dioxus::prelude::*;

use crate::views::components::common::{
    Container, Decoration, Description, DownloadIcon, PrimaryTitle,
};
use content_core::application::domain::portfolio::AboutMe;

#[component]
pub fn AboutMeSection(data: AboutMe) -> Element {
    rsx! {
        Container { id: Some("about".to_string()),
            div { class: "grid grid-cols-1 items-start gap-7 lg:grid-cols-[17rem_minmax(0,1fr)] lg:gap-10",

                div { class: "shrink-0",
                    div { class: "relative flex h-fit justify-center",
                        figure { class: "h-52 w-52 min-h-52 min-w-52 overflow-hidden rounded-full border-4 border-cyan-200/70 shadow-[0_16px_26px_-18px_rgba(15,24,36,0.45)] md:h-64 md:w-64 md:min-h-64 md:min-w-64",
                            img {
                                src: data.profile_picture().url().to_string(),
                                alt: data.profile_picture().alt().to_string(),
                                title: data.profile_picture().title().to_string(),
                                class: "block h-full w-full aspect-square rounded-full object-cover",
                                loading: "lazy",
                            }
                            figcaption { class: "hidden", "{data.profile_picture().alt()}" }
                        }
                    }

                    div { class: "mt-4 flex items-center justify-center rounded-2xl bg-primary/[0.05] px-4 py-3",
                        div { class: "pe-2",
                            div {
                                class: "font-mono text-6xl font-semibold text-primary",
                                span { "{data.years_of_experience()}" }
                            }
                        }
                        div { class: "py-2",
                            span { class: "mb-1 block text-xl font-medium text-primary", "+" }
                            p { class: "font-mono text-xs font-medium uppercase tracking-[0.11em] text-muted-foreground",
                                "Years of Experience"
                            }
                        }
                    }
                }

                div {
                    Decoration { text: data.subtitle().to_string() }
                    PrimaryTitle { text: data.title().to_string() }
                    Description { text: data.text().to_string() }

                    div { class: "mb-2 mt-4 flex flex-wrap gap-2",
                        for skill in data.skills().iter() {
                            div { class: "inline-flex items-center rounded-full border border-border/90 bg-surface-soft/60 px-4 py-2 font-mono text-sm text-zeus transition duration-150 ease-linear hover:border-accent/45 hover:bg-accent/10 hover:text-primary",
                                span { "{skill}" }
                            }
                        }
                    }

                    div { class: "mb-2 mt-4 flex justify-end",
                        a {
                            href: data.cv_document().url().to_string(),
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center justify-center rounded-xl border border-border/90 bg-surface-soft/60 px-4 py-2 font-mono text-xs font-semibold uppercase tracking-[0.09em] text-primary transition duration-150 ease-out hover:-translate-y-px hover:border-accent/40 hover:bg-accent/10 hover:text-accent",
                            span { "Download cv" }
                            DownloadIcon { class: Some("ml-1 h-4 w-4".to_string()) }
                        }
                    }
                }
            }
        }
    }
}

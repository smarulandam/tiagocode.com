use dioxus::prelude::*;

use crate::views::components::common::{
    Container, Decoration, Description, DownloadIcon, PrimaryTitle,
};
use content_core::application::domain::portfolio::AboutMe;

#[component]
pub fn AboutMeSection(data: AboutMe) -> Element {
    rsx! {
        Container { id: Some("about".to_string()),
            div { class: "lg:flex space-y-5 lg:space-x-10 lg:space-y-0",

                div { class: "shrink-0",
                    div { class: "relative flex h-fit justify-center",
                        figure { class: "h-52 w-52 min-h-52 min-w-52 overflow-hidden rounded-full md:h-64 md:w-64 md:min-h-64 md:min-w-64",
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

                    div { class: "mt-2 flex items-center justify-center",
                        div { class: "pe-2",
                            div {
                                class: "font-mono text-6xl font-semibold text-black",
                                style: "-webkit-text-fill-color: transparent; -webkit-text-stroke-width: 1.4px; -webkit-text-stroke-color: black;",
                                span { class: "counter", "{data.years_of_experience()}" }
                            }
                        }
                        div { class: "py-2",
                            span { class: "mb-1 block text-2xl font-normal", "+" }
                            p { class: "font-mono text-sm font-medium uppercase tracking-[0.5px] text-zeus",
                                "Years of Experience"
                            }
                        }
                    }
                }

                div {
                    Decoration { text: data.subtitle().to_string() }
                    PrimaryTitle { text: data.title().to_string() }
                    Description { text: data.text().to_string() }

                    div { class: "mb-2 mt-3 space-y-3",
                        for skill in data.skills().iter() {
                            div { class: "me-2 inline-block rounded-full border border-black/20 border-dashed px-4 py-2 text-zeus transition duration-100 ease-linear hover:bg-accent/70",
                                span { class: "mr-2 hidden", " " }
                                span { class: "inline-block font-mono text-sm", "{skill}" }
                            }
                        }
                    }

                    div { class: "mb-2 flex justify-end space-y-3",
                        a {
                            href: data.cv_document().url().to_string(),
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center text-sm text-zeus transition duration-[120ms] ease-out hover:text-primary",
                            span { class: "btn-text", "Download cv" }
                            DownloadIcon { class: Some("ml-1 h-4 w-4".to_string()) }
                        }
                    }
                }
            }
        }
    }
}

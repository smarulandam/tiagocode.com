use dioxus::prelude::*;

use content_core::application::domain::portfolio::AboutMe;

#[component]
pub fn AboutMeSection(data: AboutMe) -> Element {
    rsx! {
        section {
            id: "about",
            class: "section rounded-lg bg-white px-6 py-8 shadow-[0_8px_26px_0_rgba(22,24,26,0.15)] transition ease-out duration-[160ms] hover:shadow-[0_10px_30px_0_rgba(22,24,26,0.22)] md:px-8 md:py-10 lg:p-12",
            div {
                class: "lg:flex space-y-5 lg:space-x-10 lg:space-y-0",

                div { class: "shrink-0",
                    div { class: "relative flex h-fit justify-center",
                        figure {
                            class: "h-52 w-52 min-h-52 min-w-52 overflow-hidden rounded-full md:h-64 md:w-64 md:min-h-64 md:min-w-64",
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
                            p { class: "font-mono text-sm font-medium uppercase tracking-[0.5px] text-zeus", "Years of Experience" }
                        }
                    }
                }

                div {
                    p {
                        class: "relative mb-5 pt-4 font-mono text-sm font-medium uppercase tracking-wider text-accent before:pr-2 before:content-['//']",
                        "{data.subtitle()}"
                    }
                    h2 { class: "mb-2 text-4xl font-display font-semibold text-primary", "{data.title()}" }
                    p { class: "leading-7 text-zeus", "{data.text()}" }

                    div { class: "mb-2 mt-3 space-y-3",
                        for skill in data.skills().iter() {
                            div {
                                class: "me-2 inline-block rounded-full border border-black/20 border-dashed px-4 py-2 text-zeus transition duration-100 ease-linear hover:bg-accent/70",
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
                            i { class: "bi bi-cloud-download ml-1 text-base leading-none" }
                        }
                    }
                }
            }
        }
    }
}

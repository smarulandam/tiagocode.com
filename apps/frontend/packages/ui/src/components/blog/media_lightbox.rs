use dioxus::prelude::*;

use content_core::application::domain::common::Image;

#[derive(Clone, Debug, PartialEq)]
pub struct LightboxState {
    pub images: Vec<Image>,
    pub active_index: usize,
}

impl LightboxState {
    pub fn new(images: Vec<Image>, active_index: usize) -> Self {
        let safe_index = if images.is_empty() {
            0
        } else {
            active_index.min(images.len() - 1)
        };

        Self {
            images,
            active_index: safe_index,
        }
    }
}

#[component]
pub fn MediaLightbox(state: LightboxState, on_close: Callback<()>) -> Element {
    let images = state.images.clone();
    let total_images = images.len();
    let mut active_index = use_signal(|| state.active_index);

    let previous_image = move |_| {
        if total_images > 1 {
            active_index.with_mut(|index| {
                *index = if *index == 0 {
                    total_images - 1
                } else {
                    *index - 1
                };
            });
        }
    };

    let next_image = move |_| {
        if total_images > 1 {
            active_index.with_mut(|index| {
                *index = (*index + 1) % total_images;
            });
        }
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/80 px-4 py-6 backdrop-blur-sm md:px-8 md:py-8",
            role: "dialog",
            aria_modal: "true",
            aria_label: "Image preview",
            onclick: move |_| on_close.call(()),
            div {
                class: "mx-auto flex h-full w-full max-w-[1100px] flex-col justify-center gap-4",
                onclick: move |event| event.stop_propagation(),
                div { class: "flex items-center justify-between gap-4 text-white/74",
                    p { class: "font-mono text-xs uppercase tracking-[0.18em] md:text-sm",
                        if total_images > 1 {
                            "Image {active_index() + 1} of {total_images}"
                        } else {
                            "Image preview"
                        }
                    }
                    button {
                        r#type: "button",
                        class: "cursor-pointer rounded-full border border-white/18 px-4 py-2 text-sm font-medium text-white transition duration-[120ms] ease-out hover:border-white/34 hover:bg-white/10 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/30 focus-visible:ring-offset-2 focus-visible:ring-offset-black",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }

                div {
                    class: "relative flex min-h-0 flex-1 items-center justify-center overflow-hidden rounded-[1.25rem] border border-white/12 bg-[#0f1720] p-4 shadow-[0_24px_60px_rgba(0,0,0,0.35)] md:p-6",
                    if total_images > 1 {
                        button {
                            r#type: "button",
                            class: "absolute left-4 top-1/2 z-10 -translate-y-1/2 cursor-pointer rounded-full border border-white/14 bg-black/35 px-4 py-2 text-sm font-medium text-white transition duration-[120ms] ease-out hover:border-white/28 hover:bg-black/55 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/30 focus-visible:ring-offset-2 focus-visible:ring-offset-[#0f1720]",
                            onclick: previous_image,
                            "Prev"
                        }
                        button {
                            r#type: "button",
                            class: "absolute right-4 top-1/2 z-10 -translate-y-1/2 cursor-pointer rounded-full border border-white/14 bg-black/35 px-4 py-2 text-sm font-medium text-white transition duration-[120ms] ease-out hover:border-white/28 hover:bg-black/55 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/30 focus-visible:ring-offset-2 focus-visible:ring-offset-[#0f1720]",
                            onclick: next_image,
                            "Next"
                        }
                    }

                    if let Some(image) = images.get(active_index()).cloned() {
                        div { class: "flex h-full w-full flex-col items-center justify-center gap-4",
                            img {
                                class: "max-h-[75vh] w-full object-contain",
                                src: image.url().to_string(),
                                alt: image.alt().to_string(),
                            }
                            p { class: "max-w-[70ch] text-center text-sm leading-6 text-white/70",
                                "{image.alt()}"
                            }
                        }
                    }
                }
            }
        }
    }
}

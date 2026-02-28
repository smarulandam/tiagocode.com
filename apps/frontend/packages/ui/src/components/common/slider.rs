use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

use content_core::application::domain::common::Image;

use crate::components::common::Img;

static SLIDER_COUNTER: AtomicU64 = AtomicU64::new(1);

#[component]
pub fn Slider(thumbnails: Vec<Image>, images: Vec<Image>) -> Element {
    let slider_id = use_hook(|| SLIDER_COUNTER.fetch_add(1, Ordering::Relaxed));
    let identifier = slider_id.to_string();
    let main_slider_id = format!("main-slider-{identifier}");
    let thumbnail_slider_id = format!("thumbnail-slider-{identifier}");

    rsx! {
        div {
            class: "box-border relative overflow-hidden p-0 md:p-12",
            "data-slider": identifier,
            div {
                id: main_slider_id,
                class: "splide splide-wrapper mt-4 first:mt-0",
                div { class: "splide__track",
                    ul { class: "splide__list",
                        for image in images {
                            li { class: "splide__slide opacity-60 [&.is-active]:opacity-100",
                                Img {
                                    image,
                                    class: Some("h-full w-full object-cover".to_string()),
                                }
                            }
                        }
                    }
                }
            }
            div {
                id: thumbnail_slider_id,
                class: "splide splide-wrapper mt-4 first:mt-0",
                div { class: "splide__track",
                    ul { class: "splide__list",
                        for thumbnail in thumbnails {
                            li { class: "splide__slide opacity-60 [&.is-active]:opacity-100",
                                Img {
                                    image: thumbnail,
                                    class: Some("h-full w-full object-cover".to_string()),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

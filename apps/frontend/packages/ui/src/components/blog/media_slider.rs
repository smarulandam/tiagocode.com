use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::components::common::ImageView;
use content_core::application::domain::common::Image;

static SLIDER_COUNTER: AtomicU64 = AtomicU64::new(1);

#[component]
pub fn ArticleMediaSlider(
    thumbnails: Vec<Image>,
    images: Vec<Image>,
    on_open: Callback<usize>,
) -> Element {
    let slider_id = use_hook(|| SLIDER_COUNTER.fetch_add(1, Ordering::Relaxed));
    let identifier = slider_id.to_string();

    rsx! {
        div { class: "article-media", "data-slider": identifier.clone(),
            div { class: "article-media-frame",
                div { class: "splide article-slider-main", id: format!("main-slider-{identifier}"),
                    div { class: "splide__track",
                        ul { class: "splide__list",
                            for (index, image) in images.iter().cloned().enumerate() {
                                li { class: "splide__slide opacity-60 [&.is-active]:opacity-100",
                                    div { class: "article-media-viewport relative min-h-[16rem] md:min-h-[28rem]",
                                        button {
                                            r#type: "button",
                                            class: "absolute right-5 top-5 z-10 cursor-pointer rounded-full border border-black/10 bg-white/92 px-3.5 py-2 text-sm font-medium text-deepsea shadow-[0_12px_28px_rgba(36,36,36,0.08)] transition duration-[120ms] ease-out hover:border-teal/22 hover:text-teal focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal/25 focus-visible:ring-offset-2",
                                            onclick: {
                                                let on_open = on_open.clone();
                                                move |_| on_open.call(index)
                                            },
                                            "Zoom"
                                        }
                                        ImageView {
                                            image,
                                            class: "h-full w-full object-contain".to_string(),
                                            with_wrapper: false,
                                            loading: Some("lazy".to_string()),
                                            decoding: Some("async".to_string()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    class: "splide article-slider-thumbnails mt-4",
                    id: format!("thumbnail-slider-{}", slider_id),
                    div { class: "splide__track",
                        ul { class: "splide__list",
                            for image in thumbnails {
                                li { class: "splide__slide opacity-60 [&.is-active]:opacity-100",
                                    div { class: "overflow-hidden rounded-[0.85rem] bg-white/90",
                                        ImageView {
                                            image,
                                            class: "h-16 w-full object-cover md:h-20".to_string(),
                                            with_wrapper: false,
                                            loading: Some("lazy".to_string()),
                                            decoding: Some("async".to_string()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

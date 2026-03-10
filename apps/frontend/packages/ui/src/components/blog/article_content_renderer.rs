use dioxus::prelude::*;

use super::{ArticleMediaSlider, LightboxState, MediaLightbox};
use crate::components::common::{ImageView, RawHtml, UnsupportedSection};
use content_core::application::domain::article::ArticleContent;
use content_core::application::domain::common::Image;

#[component]
fn ArticleImageBlock(image: Image, on_zoom: Callback<()>) -> Element {
    rsx! {
        div { class: "article-media",
            div { class: "article-media-frame",
                button {
                    r#type: "button",
                    class: "absolute right-5 top-5 z-10 cursor-pointer rounded-full border border-black/10 bg-white/92 px-3.5 py-2 text-sm font-medium text-deepsea shadow-[0_12px_28px_rgba(36,36,36,0.08)] transition duration-[120ms] ease-out hover:border-teal/22 hover:text-teal focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal/25 focus-visible:ring-offset-2",
                    onclick: move |_| on_zoom.call(()),
                    "Zoom"
                }
                div { class: "article-media-viewport",
                    ImageView {
                        image,
                        class: "max-h-[38rem] w-full object-contain".to_string(),
                        with_wrapper: false,
                        loading: Some("lazy".to_string()),
                        decoding: Some("async".to_string()),
                    }
                }
            }
        }
    }
}

#[component]
pub fn ArticleContentRenderer(content: Vec<ArticleContent>) -> Element {
    let mut lightbox = use_signal(|| None::<LightboxState>);

    rsx! {
        div { class: "mt-7 flex flex-col gap-6 md:mt-8 md:gap-8",
            for content_block in content {
                match content_block {
                    ArticleContent::Image(image) => {
                        let zoom_image = image.clone();
                        rsx! {
                            ArticleImageBlock {
                                image,
                                on_zoom: Callback::new(move |_| {
                                    lightbox.set(Some(LightboxState::new(vec![zoom_image.clone()], 0)));
                                }),
                            }
                        }
                    }
                    ArticleContent::Text(text) => rsx! {
                        RawHtml {
                            html: text.to_string(),
                            class: "article-prose".to_string(),
                        }
                    },
                    ArticleContent::Slider(thumbnails, images) => {
                        let gallery_images = images.clone();
                        rsx! {
                            ArticleMediaSlider {
                                thumbnails: thumbnails.clone(),
                                images: images.clone(),
                                on_open: Callback::new(move |index: usize| {
                                    lightbox.set(Some(LightboxState::new(gallery_images.clone(), index)));
                                }),
                            }
                        }
                    }
                    ArticleContent::Unknown => rsx! {
                        UnsupportedSection {}
                    },
                }
            }
        }

        if let Some(state) = lightbox() {
            MediaLightbox {
                state,
                on_close: Callback::new(move |_| lightbox.set(None)),
            }
        }
    }
}

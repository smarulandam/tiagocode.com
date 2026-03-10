use dioxus::prelude::*;

use content_core::application::domain::common::Image;

#[component]
pub fn ImageView(
    image: Image,
    #[props(default = "".to_string())] class: String,
    #[props(default = true)] with_wrapper: bool,
    #[props(default)] loading: Option<String>,
    #[props(default)] decoding: Option<String>,
) -> Element {
    let src = image.url().to_string();
    let alt = image.alt().to_string();
    let width = *image.width() as u32;
    let height = *image.height() as u32;
    let loading = loading.unwrap_or_default();
    let decoding = decoding.unwrap_or_default();

    if with_wrapper {
        rsx! {
            figure {
                img {
                    class: class.clone(),
                    src: src.clone(),
                    alt: alt.clone(),
                    width,
                    height,
                    loading,
                    decoding,
                }
                figcaption { class: "hidden", "{alt}" }
            }
        }
    } else {
        rsx! {
            img {
                class,
                src,
                alt,
                width,
                height,
                loading,
                decoding,
            }
        }
    }
}

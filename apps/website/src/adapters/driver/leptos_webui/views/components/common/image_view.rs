use leptos::either::Either;
use leptos::prelude::*;

use crate::application::domain::common::Image;

#[component]
pub fn ImageView(
    image: Image,
    #[prop(default = "")] class: &'static str,
    #[prop(default = true)] with_wrapper: bool,
    #[prop(default = None)] loading: Option<&'static str>,
    #[prop(default = None)] decoding: Option<&'static str>,
) -> impl IntoView {
    let src = image.url().to_string();
    let alt = image.alt().to_string();
    let width = *image.width();
    let height = *image.height();

    if with_wrapper {
        Either::Left(view! {
            <figure>
                <img
                    class=class
                    src=src.clone()
                    alt=alt.clone()
                    width=width
                    height=height
                    loading=loading
                    decoding=decoding
                />
                <figcaption class="hidden">
                    {alt.clone()}
                </figcaption>
            </figure>
        })
    } else {
        Either::Right(view! {
            <img
                class=class
                src=src
                alt=alt
                width=width
                height=height
                loading=loading
                decoding=decoding
            />
        })
    }
}

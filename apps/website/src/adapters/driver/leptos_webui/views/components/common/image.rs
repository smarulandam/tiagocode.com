use leptos::either::Either;
use leptos::prelude::*;

use crate::application::domain::common::Image;

#[component]
pub fn Img(
    image: Image,
    #[prop(default = "")] class: &'static str,
    #[prop(default = true)] with_wrapper: bool,
) -> impl IntoView {
    if with_wrapper {
        Either::Left(view! {
            <figure>
                <img class=class src=image.url().to_string() alt=image.alt().to_string() />
                <figcaption class="hidden">
                    {image.alt().to_string()}
                </figcaption>
            </figure>
        })
    } else {
        Either::Right(view! {
            <img class=class src=image.url().to_string() alt=image.alt().to_string() />
        })
    }
}

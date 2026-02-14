use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::Img;
use crate::application::domain::common::Image;

#[component]
pub fn Slider(thumbnails: Vec<Image>, images: Vec<Image>) -> impl IntoView {
    let mut buffer = [0u8; 4];
    getrandom::fill(&mut buffer).unwrap();

    let id = u32::from_le_bytes(buffer);

    view! {
        <div class="p-0 md:p-12 box-border overflow-hidden relative" data-slider=id>
            <div class="splide splide-wrapper mt-4 first:mt-0" id=format!("main-slider-{}", id)>
                <div class="splide__track">
                    <ul class="splide__list">
                        {images.clone().into_iter().map(|img| {view! {
                            <li class="splide__slide opacity-60 [&.is-active]:opacity-100">
                                <Img image=img.clone() class="w-full h-full object-cover" with_wrapper=false />
                            </li>
                        }}).collect_view()}
                    </ul>
                </div>
            </div>
            <div class="splide splide-wrapper mt-4 first:mt-0" id=format!("thumbnail-slider-{}", id)>
                <div class="splide__track">
                    <ul class="splide__list">
                        {thumbnails.clone().into_iter().map(|img| {view! {
                            <li class="splide__slide opacity-60 [&.is-active]:opacity-100">
                                <Img image=img.clone() class="w-full h-full object-cover" with_wrapper=false />
                            </li>
                        }}).collect_view()}
                    </ul>
                </div>
            </div>
        </div>
    }
}

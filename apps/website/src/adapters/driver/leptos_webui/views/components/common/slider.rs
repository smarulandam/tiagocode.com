use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::Img;
use crate::application::domain::common::Image;

#[component]
pub fn Slider(
    thumbnails: Vec<Image>,
    images: Vec<Image>,
    #[prop(into)] on_open: Callback<usize>,
) -> impl IntoView {
    let mut buffer = [0u8; 4];
    getrandom::fill(&mut buffer).unwrap();

    let id = u32::from_le_bytes(buffer);

    view! {
        <div class="article-media" data-slider=id>
            <div class="article-media-frame">
                <div class="splide article-slider-main" id=format!("main-slider-{}", id)>
                    <div class="splide__track">
                        <ul class="splide__list">
                            {images
                                .clone()
                                .into_iter()
                                .enumerate()
                                .map(|(index, img)| {
                                    let on_open = on_open;

                                    view! {
                                        <li class="splide__slide opacity-60 [&.is-active]:opacity-100">
                                            <div class="article-media-viewport relative min-h-[16rem] md:min-h-[28rem]">
                                                <button
                                                    type="button"
                                                    class="absolute right-5 top-5 z-10 cursor-pointer rounded-full border border-black/10 bg-white/92 px-3.5 py-2 text-sm font-medium text-deepsea shadow-[0_12px_28px_rgba(36,36,36,0.08)] transition duration-[120ms] ease-out hover:border-teal/22 hover:text-teal focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal/25 focus-visible:ring-offset-2"
                                                    on:click=move |_| on_open.run(index)
                                                >
                                                    "Zoom"
                                                </button>
                                                <Img
                                                    image=img.clone()
                                                    class="h-full w-full object-contain"
                                                    with_wrapper=false
                                                />
                                            </div>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </div>
                </div>

                <div class="splide article-slider-thumbnails mt-4" id=format!("thumbnail-slider-{}", id)>
                    <div class="splide__track">
                        <ul class="splide__list">
                            {thumbnails.clone().into_iter().map(|img| {view! {
                                <li class="splide__slide opacity-60 [&.is-active]:opacity-100">
                                    <div class="overflow-hidden rounded-[0.85rem] bg-white/90">
                                        <Img image=img.clone() class="h-16 w-full object-cover md:h-20" with_wrapper=false />
                                    </div>
                                </li>
                            }}).collect_view()}
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

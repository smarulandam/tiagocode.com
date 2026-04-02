use leptos::either::EitherOf4;
use leptos::prelude::*;

use super::{ArticleMediaSlider, LightboxState, MediaLightbox};
use crate::adapters::driver::leptos_webui::views::components::common::{
    ImageView, RawHtml, UnsupportedSection,
};
use crate::application::domain::article::ArticleContent;
use crate::application::domain::common::Image;

#[component]
fn ArticleImageBlock(image: Image, #[prop(into)] on_zoom: Callback<()>) -> impl IntoView {
    view! {
        <div class="article-media article-media--plain">
            <div class="article-media-frame">
                <button
                    type="button"
                    class="absolute right-5 top-5 z-10 cursor-pointer rounded-full border border-black/10 bg-white/92 px-3.5 py-2 text-sm font-medium text-deepsea shadow-[0_12px_28px_rgba(36,36,36,0.08)] transition duration-[120ms] ease-out hover:border-teal/22 hover:text-teal focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-teal/25 focus-visible:ring-offset-2"
                    on:click=move |_| on_zoom.run(())
                >
                    "Zoom"
                </button>
                <div class="article-media-viewport">
                    <ImageView
                        image=image
                        class="max-h-[38rem] w-full object-contain"
                        with_wrapper=false
                        loading=Some("lazy")
                        decoding=Some("async")
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ArticleContentRenderer(content: Vec<ArticleContent>) -> impl IntoView {
    let (lightbox, set_lightbox) = signal(None::<LightboxState>);

    view! {
        <div class="article-content mt-7 flex flex-col gap-6 md:mt-8 md:gap-8" data-article-content="true">
            {content
                .into_iter()
                .enumerate()
                .map(|(index, content_block)| match content_block {
                    ArticleContent::Image(image) => {
                        let zoom_image = image.clone();

                        EitherOf4::A(view! {
                            <ArticleImageBlock
                                image=image
                                on_zoom=move |_| {
                                    set_lightbox
                                        .set(Some(LightboxState::new(vec![zoom_image.clone()], 0)));
                                }
                            />
                        })
                    }
                    ArticleContent::Text(text) => EitherOf4::B(view! {
                        <RawHtml html=text.to_string() class="article-prose" />
                    }),
                    ArticleContent::Slider(thumbnails, images) => {
                        let gallery_images = images.clone();

                        EitherOf4::C(view! {
                            <ArticleMediaSlider
                                slider_id=format!("article-slider-{}", index)
                                thumbnails=thumbnails.clone()
                                images=images.clone()
                                on_open=move |index| {
                                    set_lightbox
                                        .set(Some(LightboxState::new(gallery_images.clone(), index)));
                                }
                            />
                        })
                    }
                    _ => EitherOf4::D(view! { <UnsupportedSection /> }),
                })
                .collect_view()}
        </div>

        <Show when=move || lightbox.get().is_some()>
            {move || {
                lightbox.get().map(|state| {
                    view! {
                        <MediaLightbox state=state on_close=move |_| set_lightbox.set(None) />
                    }
                })
            }}
        </Show>
    }
}

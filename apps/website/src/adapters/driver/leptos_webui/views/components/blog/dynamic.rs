use leptos::either::EitherOf4;
use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::application::domain::article::ArticleContent;

#[component]
pub fn DynamicContent(content: Vec<ArticleContent>) -> impl IntoView {
    view! {
        {content
            .into_iter()
            .map(|content| {
                match content {
                    ArticleContent::Image(img) => {
                        EitherOf4::A(view! { <Img image=img.clone() class="mt-6" /> })
                    },
                    ArticleContent::Text(text) => {
                        EitherOf4::B(view! { <RawHtml html=text.to_string() class="mt-6"/> })
                    },
                    ArticleContent::Slider(thumbnails, images) => {
                        EitherOf4::C(view! { <Slider thumbnails=thumbnails.clone() images=images.clone() /> })
                    },
                    _ => EitherOf4::D(view! {<MissingSection />})
                }
            }).collect_view()
        }
    }
}

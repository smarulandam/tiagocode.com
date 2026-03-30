use leptos::{ev, prelude::*};

use crate::application::domain::common::Image;

#[derive(Clone, Debug)]
pub struct LightboxState {
    pub images: Vec<Image>,
    pub active_index: usize,
}

impl LightboxState {
    pub fn new(images: Vec<Image>, active_index: usize) -> Self {
        let safe_index = if images.is_empty() {
            0
        } else {
            active_index.min(images.len() - 1)
        };

        Self {
            images,
            active_index: safe_index,
        }
    }
}

#[component]
pub fn MediaLightbox(state: LightboxState, #[prop(into)] on_close: Callback<()>) -> impl IntoView {
    let images = state.images.clone();
    let total_images = images.len();

    let (active_index, set_active_index) = signal(state.active_index);

    let previous_image = {
        let set_active_index = set_active_index;

        move |_| {
            if total_images > 1 {
                set_active_index.update(|index| {
                    *index = if *index == 0 {
                        total_images - 1
                    } else {
                        *index - 1
                    };
                });
            }
        }
    };

    let next_image = {
        let set_active_index = set_active_index;

        move |_| {
            if total_images > 1 {
                set_active_index.update(|index| {
                    *index = (*index + 1) % total_images;
                });
            }
        }
    };

    let key_listener = {
        let set_active_index = set_active_index;
        let on_close = on_close;

        window_event_listener(ev::keydown, move |event| match event.key().as_str() {
            "Escape" => on_close.run(()),
            "ArrowLeft" if total_images > 1 => {
                set_active_index.update(|index| {
                    *index = if *index == 0 {
                        total_images - 1
                    } else {
                        *index - 1
                    };
                });
            }
            "ArrowRight" if total_images > 1 => {
                set_active_index.update(|index| {
                    *index = (*index + 1) % total_images;
                });
            }
            _ => {}
        })
    };

    on_cleanup(move || key_listener.remove());

    let current_image = move || images.get(active_index.get()).cloned();

    view! {
        <div
            class="fixed inset-0 z-50 bg-black/80 px-4 py-6 backdrop-blur-sm md:px-8 md:py-8"
            role="dialog"
            aria-modal="true"
            aria-label="Image preview"
            on:click=move |_| on_close.run(())
        >
            <div
                class="mx-auto flex h-full w-full max-w-[1100px] flex-col justify-center gap-4"
                on:click=|event| event.stop_propagation()
            >
                <div class="flex items-center justify-between gap-4 text-white/74">
                    <p class="font-mono text-xs uppercase tracking-[0.18em] md:text-sm">
                        {move || {
                            if total_images > 1 {
                                format!("Image {} of {}", active_index.get() + 1, total_images)
                            } else {
                                "Image preview".to_string()
                            }
                        }}
                    </p>
                    <button
                        type="button"
                        class="cursor-pointer rounded-full border border-white/18 px-4 py-2 text-sm font-medium text-white transition duration-[120ms] ease-out hover:border-white/34 hover:bg-white/10 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/30 focus-visible:ring-offset-2 focus-visible:ring-offset-black"
                        on:click=move |_| on_close.run(())
                    >
                        "Close"
                    </button>
                </div>

                <div class="relative flex min-h-0 flex-1 items-center justify-center overflow-hidden rounded-[1.25rem] border border-white/12 bg-[#0f1720] p-4 shadow-[0_24px_60px_rgba(0,0,0,0.35)] md:p-6">
                    <Show when=move || { total_images > 1 }>
                        <button
                            type="button"
                            class="absolute left-4 top-1/2 z-10 -translate-y-1/2 cursor-pointer rounded-full border border-white/14 bg-black/35 px-4 py-2 text-sm font-medium text-white transition duration-[120ms] ease-out hover:border-white/28 hover:bg-black/55 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/30 focus-visible:ring-offset-2 focus-visible:ring-offset-[#0f1720]"
                            on:click=previous_image
                        >
                            "Prev"
                        </button>
                        <button
                            type="button"
                            class="absolute right-4 top-1/2 z-10 -translate-y-1/2 cursor-pointer rounded-full border border-white/14 bg-black/35 px-4 py-2 text-sm font-medium text-white transition duration-[120ms] ease-out hover:border-white/28 hover:bg-black/55 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/30 focus-visible:ring-offset-2 focus-visible:ring-offset-[#0f1720]"
                            on:click=next_image
                        >
                            "Next"
                        </button>
                    </Show>

                    {move || {
                        current_image().map(|image| {
                            view! {
                                <div class="flex h-full w-full flex-col items-center justify-center gap-4">
                                    <img
                                        class="max-h-[75vh] w-full object-contain"
                                        src=image
                                            .url_high_resolution()
                                            .as_ref()
                                            .unwrap_or(image.url())
                                            .to_string()
                                        alt=image.alt().to_string()
                                    />
                                    <p class="max-w-[70ch] text-center text-sm leading-6 text-white/70">
                                        {image.alt().to_string()}
                                    </p>
                                </div>
                            }
                        })
                    }}
                </div>
            </div>
        </div>
    }
}

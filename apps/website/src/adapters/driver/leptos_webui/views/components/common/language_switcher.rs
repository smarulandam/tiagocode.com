use leptos::{ev, prelude::*, web_sys};
use leptos_router::hooks::use_params_map;
use wasm_bindgen::JsCast;

#[component]
pub fn LanguageSwitcher() -> impl IntoView {
    let params = use_params_map();
    let (is_open, set_is_open) = signal(false);
    let active_language = params.read_untracked().get("lang").unwrap_or("en".into());
    let is_english_active = active_language == "en";
    let is_spanish_active = active_language == "es";

    let close_on_focus_out = move |event: ev::FocusEvent| {
        let Some(current_target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
        else {
            set_is_open.set(false);
            return;
        };

        let Some(next_target) = event
            .related_target()
            .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
        else {
            set_is_open.set(false);
            return;
        };

        if !current_target.contains(Some(&next_target)) {
            set_is_open.set(false);
        }
    };

    let navigate_to = move |path: &'static str| {
        move |_| {
            set_is_open.set(false);

            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href(path);
            }
        }
    };

    view! {
        <div
            class="relative"
            data-language-switcher="true"
            on:focusout=close_on_focus_out
            on:keydown=move |event: ev::KeyboardEvent| {
                if event.key() == "Escape" {
                    set_is_open.set(false);
                }
            }
        >
            <button
                type="button"
                class="flex cursor-pointer items-center gap-1"
                aria-label="Change language"
                aria-haspopup="menu"
                aria-expanded=move || if is_open.get() { "true" } else { "false" }
                on:click=move |_| set_is_open.update(|value| *value = !*value)
            >
                <span class="sr-only">Change language</span>
                <svg
                    class="size-[1.15rem]"
                    viewBox="0 0 16 16"
                    fill="currentColor"
                    xmlns="http://www.w3.org/2000/svg"
                    aria-hidden="true"
                >
                    <path
                        d="M4.545 6.714 4.11 8H3l1.862-5h1.284L8 8H6.833l-.435-1.286H4.545zm1.634-.736L5.5 3.956h-.049l-.679 2.022H6.18z"
                    />
                    <path
                        d="M0 2a2 2 0 0 1 2-2h7a2 2 0 0 1 2 2v3h3a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-3H2a2 2 0 0 1-2-2V2zm2-1a1 1 0 0 0-1 1v7a1 1 0 0 0 1 1h7a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H2zm7.138 9.995c.193.301.402.583.63.846-.748.575-1.673 1.001-2.768 1.292.178.217.451.635.555.867 1.125-.359 2.08-.844 2.886-1.494.777.665 1.739 1.165 2.93 1.472.133-.254.414-.673.629-.89-1.125-.253-2.057-.694-2.82-1.284.681-.747 1.222-1.651 1.621-2.757H14V8h-3v1.047h.765c-.318.844-.74 1.546-1.272 2.13a6.066 6.066 0 0 1-.415-.492 1.988 1.988 0 0 1-.94.31z"
                    />
                </svg>
                <svg
                    class="h-3 w-3"
                    viewBox="0 0 12 12"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    aria-hidden="true"
                >
                    <path
                        d="M2.25 4.5 6 8.25 9.75 4.5"
                        stroke="currentColor"
                        stroke-width="1.4"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    />
                </svg>
            </button>

            <div
                class="absolute right-0 top-[calc(100%+0.75rem)] z-30 w-[12rem] rounded-lg border border-black/8 bg-white p-2 shadow-smoke-shadow"
                class:hidden=move || !is_open.get()
            >
                <div class="flex flex-col gap-1" role="menu" aria-label="Language options">
                    <button
                        type="button"
                        role="menuitem"
                        data-language-option="en"
                        class=move || {
                            format!(
                                "flex w-full cursor-pointer items-center gap-3 rounded-lg border px-3 py-2 text-left transition ease-out duration-[160ms] focus:outline-hidden focus:ring-2 focus:ring-teal/18 {}",
                                if is_english_active {
                                    "border-teal bg-teal text-white shadow-smoke-shadow"
                                } else {
                                    "border-transparent bg-smoke/75 text-zeus hover:border-teal/18 hover:bg-teal/6 hover:text-teal"
                                }
                            )
                        }
                        on:click=navigate_to("/")
                    >
                        <svg
                            class="h-5 w-7 shrink-0 rounded-[0.32rem] border border-black/8 shadow-[0_2px_10px_rgba(22,24,26,0.08)]"
                            viewBox="0 0 28 20"
                            xmlns="http://www.w3.org/2000/svg"
                            aria-hidden="true"
                        >
                            <rect width="28" height="20" rx="2.5" fill="#B22234" />
                            <path
                                d="M0 2H28M0 5H28M0 8H28M0 11H28M0 14H28M0 17H28"
                                stroke="#fff"
                                stroke-width="2"
                            />
                            <rect width="12.2" height="10.5" rx="2.5" fill="#3C3B6E" />
                            <path
                                d="M2.15 2.2H10.1M2.15 4.45H10.1M2.15 6.7H10.1M2.15 8.95H10.1"
                                stroke="#fff"
                                stroke-width="0.8"
                                stroke-linecap="round"
                                stroke-dasharray="0.1 1.65"
                            />
                        </svg>
                        <span class="min-w-0 flex-1">
                            <span class="block font-medium leading-tight">English</span>
                        </span>
                        <span
                            class=move || {
                                format!(
                                    "font-mono text-[0.62rem] uppercase tracking-[0.18em] {}",
                                    if is_english_active {
                                        "text-white/78"
                                    } else {
                                        "text-teal/72"
                                    }
                                )
                            }
                        >
                            {move || if is_english_active { "Active" } else { "" }}
                        </span>
                    </button>

                    <button
                        type="button"
                        role="menuitem"
                        data-language-option="es"
                        class=move || {
                            format!(
                                "flex w-full cursor-pointer items-center gap-3 rounded-lg border px-3 py-2 text-left transition ease-out duration-[160ms] focus:outline-hidden focus:ring-2 focus:ring-teal/18 {}",
                                if is_spanish_active {
                                    "border-teal bg-teal text-white shadow-smoke-shadow"
                                } else {
                                    "border-transparent bg-smoke/75 text-zeus hover:border-teal/18 hover:bg-teal/6 hover:text-teal"
                                }
                            )
                        }
                        on:click=navigate_to("/es")
                    >
                        <svg
                            class="h-5 w-7 shrink-0 rounded-[0.32rem] border border-black/8 shadow-[0_2px_10px_rgba(22,24,26,0.08)]"
                            viewBox="0 0 28 20"
                            xmlns="http://www.w3.org/2000/svg"
                            aria-hidden="true"
                        >
                            <rect width="28" height="20" rx="2.5" fill="#AA151B" />
                            <rect y="5" width="28" height="10" fill="#F1BF00" />
                            <path
                                d="M7.6 6.35V13.65M5.9 7.55H9.3M6.35 12.45H8.85M6.65 8.35V11.65"
                                stroke="#AA151B"
                                stroke-width="0.85"
                                stroke-linecap="round"
                            />
                        </svg>
                        <span class="min-w-0 flex-1">
                            <span class="block font-medium leading-tight">Español</span>
                        </span>
                        <span
                            class=move || {
                                format!(
                                    "font-mono text-[0.62rem] uppercase tracking-[0.18em] {}",
                                    if is_spanish_active {
                                        "text-white/78"
                                    } else {
                                        "text-teal/72"
                                    }
                                )
                            }
                        >
                            {move || if is_spanish_active { "Active" } else { "" }}
                        </span>
                    </button>
                </div>
            </div>
        </div>
    }
}

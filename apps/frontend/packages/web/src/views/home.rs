use dioxus::prelude::*;
use ui::{Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            class: "mx-auto flex w-full max-w-6xl flex-col gap-8 px-4 py-8 sm:gap-10 sm:py-10",
            section {
                class: "mx-auto w-full max-w-5xl rounded-2xl border border-slate-200 bg-white/80 p-5 shadow-sm backdrop-blur sm:p-6",
                div {
                    class: "flex flex-wrap items-center gap-2.5",
                    span {
                        class: "rounded-full bg-slate-900 px-2.5 py-1 text-xs font-semibold uppercase tracking-wide text-white",
                        "Setup"
                    }
                    p {
                        class: "text-sm leading-relaxed text-slate-700",
                        "Tailwind is active and scanned across `web/src` + `ui/src`."
                    }
                }
            }
            Hero {}
            Echo {}
        }
    }
}

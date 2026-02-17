use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        main {
            class: "mx-auto w-full max-w-6xl px-4 py-8 sm:py-10",
            article {
                class: "mx-auto max-w-5xl overflow-hidden rounded-3xl border border-slate-200/70 bg-white/90 shadow-[0_28px_70px_-36px_rgba(15,23,42,0.55)] backdrop-blur",
                div {
                    class: "border-b border-slate-200/80 bg-gradient-to-r from-slate-900 via-slate-800 to-slate-900 p-7 text-white sm:p-10",
                    p {
                        class: "text-xs font-semibold uppercase tracking-[0.2em] text-sky-200",
                        "Article"
                    }
                    h1 {
                        class: "mt-2 font-display text-3xl font-semibold leading-tight sm:text-4xl",
                        "This is blog #{id}!"
                    }
                    p {
                        class: "mt-3 max-w-2xl text-sm text-slate-200 sm:text-base",
                        "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to route components."
                    }
                }
                div {
                    class: "space-y-8 p-7 sm:p-10",
                    div {
                        class: "grid gap-4 sm:grid-cols-3",
                        div {
                            class: "rounded-xl border border-slate-200 bg-slate-50 p-4",
                            p { class: "text-xs uppercase tracking-wide text-slate-500", "Route param" }
                            p { class: "mt-1 text-sm font-semibold text-slate-900", "id = {id}" }
                        }
                        div {
                            class: "rounded-xl border border-slate-200 bg-slate-50 p-4",
                            p { class: "text-xs uppercase tracking-wide text-slate-500", "Framework" }
                            p { class: "mt-1 text-sm font-semibold text-slate-900", "Dioxus 0.7" }
                        }
                        div {
                            class: "rounded-xl border border-slate-200 bg-slate-50 p-4",
                            p { class: "text-xs uppercase tracking-wide text-slate-500", "Styling" }
                            p { class: "mt-1 text-sm font-semibold text-slate-900", "Tailwind CSS" }
                        }
                    }
                    div {
                        class: "flex flex-wrap items-center justify-between gap-4 border-t border-slate-200 pt-6",
                        Link {
                            class: "rounded-xl border border-slate-300 bg-white px-4 py-2 text-sm font-semibold text-slate-700 transition hover:-translate-y-0.5 hover:border-sky-400 hover:text-sky-700",
                            to: Route::Blog { id: id - 1 },
                            "← Previous"
                        }
                        span {
                            class: "rounded-full border border-slate-200 bg-slate-50 px-3 py-1 text-xs font-semibold uppercase tracking-wide text-slate-600",
                            "Post #{id}"
                        }
                        Link {
                            class: "rounded-xl bg-slate-900 px-4 py-2 text-sm font-semibold text-white transition hover:-translate-y-0.5 hover:bg-slate-800",
                            to: Route::Blog { id: id + 1 },
                            "Next →"
                        }
                    }
                }
            }
        }
    }
}

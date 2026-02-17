use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            class: "relative mx-auto w-full max-w-5xl overflow-hidden rounded-3xl border border-slate-200/70 bg-slate-950 p-7 text-slate-100 shadow-[0_24px_70px_-28px_rgba(15,23,42,0.6)] sm:p-10 lg:p-12",
            div { class: "pointer-events-none absolute -right-24 -top-24 h-64 w-64 rounded-full bg-sky-400/25 blur-3xl" }
            div { class: "pointer-events-none absolute -bottom-28 -left-16 h-72 w-72 rounded-full bg-amber-300/20 blur-3xl" }
            div {
                class: "relative grid items-center gap-10 lg:grid-cols-[1.15fr_0.85fr]",
                div {
                    class: "space-y-6",
                    p {
                        class: "inline-flex rounded-full border border-sky-300/30 bg-sky-400/10 px-3 py-1 text-xs font-semibold uppercase tracking-[0.2em] text-sky-200",
                        "Dioxus 0.7"
                    }
                    h1 {
                        class: "font-display text-3xl font-semibold leading-tight sm:text-4xl",
                        "Build fullstack Rust apps without frontend chaos."
                    }
                    p {
                        class: "max-w-xl text-sm text-slate-300 sm:text-base",
                        "A cleaner starter UI: stronger typography, better spacing, and reusable Tailwind primitives that still feel fast to iterate."
                    }
                    div {
                        class: "flex flex-wrap gap-3",
                        a {
                            class: "rounded-xl bg-white px-4 py-2 text-sm font-semibold text-slate-900 transition hover:bg-slate-200",
                            href: "https://dioxuslabs.com/learn/0.7/",
                            "Open Docs"
                        }
                        a {
                            class: "rounded-xl border border-slate-600 px-4 py-2 text-sm font-semibold text-slate-100 transition hover:border-slate-400 hover:bg-slate-800",
                            href: "https://dioxuslabs.com/awesome",
                            "Community Packages"
                        }
                    }
                }
                div {
                    class: "rounded-2xl border border-slate-700/80 bg-slate-900/60 p-5 shadow-inner",
                    img {
                        class: "h-auto w-full rounded-xl bg-white p-2",
                        src: HEADER_SVG
                    }
                }
            }
            div {
                class: "relative mt-8 grid gap-3 sm:grid-cols-2 lg:grid-cols-3",
                a {
                    class: "rounded-xl border border-slate-700/70 bg-slate-900/70 px-3.5 py-2.5 text-sm font-medium text-slate-200 transition hover:border-sky-300/50 hover:text-sky-100",
                    href: "https://dioxuslabs.com/learn/0.7/",
                    "Guide"
                }
                a {
                    class: "rounded-xl border border-slate-700/70 bg-slate-900/70 px-3.5 py-2.5 text-sm font-medium text-slate-200 transition hover:border-sky-300/50 hover:text-sky-100",
                    href: "https://dioxuslabs.com/awesome",
                    "Awesome Dioxus"
                }
                a {
                    class: "rounded-xl border border-slate-700/70 bg-slate-900/70 px-3.5 py-2.5 text-sm font-medium text-slate-200 transition hover:border-sky-300/50 hover:text-sky-100",
                    href: "https://github.com/dioxus-community/",
                    "Community"
                }
                a {
                    class: "rounded-xl border border-slate-700/70 bg-slate-900/70 px-3.5 py-2.5 text-sm font-medium text-slate-200 transition hover:border-sky-300/50 hover:text-sky-100",
                    href: "https://github.com/DioxusLabs/sdk",
                    "SDK"
                }
                a {
                    class: "rounded-xl border border-slate-700/70 bg-slate-900/70 px-3.5 py-2.5 text-sm font-medium text-slate-200 transition hover:border-sky-300/50 hover:text-sky-100",
                    href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "VSCode Extension"
                }
                a {
                    class: "rounded-xl border border-slate-700/70 bg-slate-900/70 px-3.5 py-2.5 text-sm font-medium text-slate-200 transition hover:border-sky-300/50 hover:text-sky-100",
                    href: "https://discord.gg/XgGxMSkvUM",
                    "Discord"
                }
            }
        }
    }
}

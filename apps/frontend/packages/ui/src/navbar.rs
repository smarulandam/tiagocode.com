use dioxus::prelude::*;

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        header { class: "sticky top-0 z-30 border-b border-slate-200/80 bg-white/75 backdrop-blur-xl",
            div { class: "mx-auto flex w-full max-w-6xl items-center gap-4 px-4 py-3",
                a { class: "inline-flex items-center gap-3", href: "/",
                    span { class: "inline-flex h-10 w-10 items-center justify-center rounded-xl bg-slate-900 text-sm font-bold text-white shadow-sm",
                        "DX"
                    }
                    span { class: "leading-tight",
                        strong { class: "block font-display text-sm font-semibold text-slate-900",
                            "Dioxus Starter"
                        }
                        small { class: "block text-xs text-slate-500", "Tailwind + Fullstack" }
                    }
                }
                nav { class: "ml-auto flex items-center gap-2 rounded-xl border border-slate-200 bg-white/90 p-1 shadow-sm",
                    {children}
                }
            }
        }
    }
}

use dioxus::prelude::*;

use crate::components::common::{LanguageSelector, Menu};
use content_core::application::domain::layout::MenuTree;

#[component]
pub fn Navbar(
    main_menu: MenuTree,
    social_menu: MenuTree,
) -> Element {
    let mut open_mobile_menu = use_signal(|| false);

    rsx! {
        nav { class: "sticky top-0 z-40 w-full border-b border-border/80 bg-white shadow-[0_10px_24px_-24px_rgba(15,24,36,0.45)] backdrop-blur-[10px] transition duration-[220ms] ease-out",
            div { class: "mx-auto flex min-h-[4.5rem] max-w-[1320px] flex-wrap items-center justify-between gap-2 px-5 py-4 xl:px-0",

                a {
                    class: "flex items-center space-x-3 text-primary/90",
                    href: "/",
                    img {
                        class: "h-8",
                        alt: "Tiagocode Logo",
                        src: asset!("/assets/images/logo_teal.svg"),
                    }
                    span { class: "hidden self-center whitespace-nowrap font-display text-2xl font-semibold uppercase tracking-[0.14em] text-primary/90 md:block",
                        "Tiagocode"
                    }
                }

                div { class: "flex items-center gap-1md:gap-3",
                    div { class: "hidden md:order-1 md:block",
                        Menu {
                            menu: main_menu.clone(),
                            container_class: Some("flex items-center gap-2".to_string()),
                            item_class: Some("block px-1 py-0 uppercase".to_string()),
                            anchor_class: Some(
                                "inline-flex items-center rounded-lg px-[1.05rem] py-[0.68rem] font-mono text-[1rem] font-extrabold uppercase leading-[1.2] tracking-[0.07em] text-foreground/90 transition duration-150 ease-out hover:-translate-y-px hover:bg-accent/10 hover:text-primary"
                                    .to_string(),
                            ),
                        }
                    }
                    div { class: "hidden md:order-2 md:block",
                        LanguageSelector {
                            container_class: "relative".to_string(),
                        }
                    }
                    div { class: "md:order-2",
                        Menu {
                            menu: social_menu.clone(),
                            container_class: Some("flex items-center gap-2".to_string()),
                            item_class: Some("block py-0 px-0".to_string()),
                            anchor_class: Some(
                                "inline-flex items-center justify-center rounded-xl bg-transparent text-foreground/90 transition duration-150 ease-out hover:-translate-y-px hover:text-primary"
                                    .to_string(),
                            ),
                        }
                    }
                }

                div { class: "block md:hidden",
                    button {
                        r#type: "button",
                        class: "inline-flex h-10 w-10 items-center justify-center rounded-xl border border-border/80 bg-white/80 p-2 text-sm text-muted-foreground shadow-sm backdrop-blur-sm hover:bg-white hover:text-foreground focus:outline-none focus:ring-2 focus:ring-accent/40",
                        aria_controls: "mobile-menu",
                        aria_expanded: open_mobile_menu().to_string(),
                        onclick: move |_| open_mobile_menu.set(!open_mobile_menu()),
                        span { class: "sr-only", "Open main menu" }
                        svg {
                            class: "h-5 w-5",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 17 14",
                            path {
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M1 1h15M1 7h15M1 13h15",
                            }
                        }
                    }
                }

                div {
                    id: "mobile-menu",
                    class: if open_mobile_menu() { "w-full md:hidden md:w-auto" } else { "hidden w-full md:hidden md:w-auto" },
                    div { class: "mt-4 rounded-xl border border-border bg-[rgba(250,252,252,0.98)] p-4 shadow-[0_18px_30px_-28px_rgba(8,15,30,0.85)]",
                        LanguageSelector {
                            container_class: "relative mb-3 flex justify-end".to_string(),
                            button_class: "inline-flex h-10 w-10 cursor-pointer items-center justify-center rounded-lg border border-border/80 bg-white/90 text-foreground/85 shadow-sm transition duration-150 ease-out hover:border-accent/60 hover:text-primary focus:outline-none focus:ring-2 focus:ring-accent/35".to_string(),
                            menu_class: "absolute right-0 top-[calc(100%+0.5rem)] z-20 min-w-[10.5rem] rounded-lg border border-border/80 bg-white p-1.5 shadow-[0_14px_24px_-18px_rgba(8,15,30,0.8)]".to_string(),
                        }
                        Menu {
                            menu: main_menu,
                            container_class: Some(
                                "flex flex-col gap-1 md:mt-0 md:flex-row md:space-x-8 md:border-0 md:bg-transparent md:p-0 rtl:space-x-reverse"
                                    .to_string(),
                            ),
                            item_class: Some(
                                "block rounded-md px-0 py-0 text-gray-900 uppercase md:border-0 md:p-0"
                                    .to_string(),
                            ),
                            anchor_class: Some(
                                "block rounded-lg px-3 py-2 font-mono text-sm font-semibold uppercase tracking-[0.09em] text-foreground/90 transition duration-150 ease-out hover:bg-accent/10 hover:text-primary"
                                    .to_string(),
                            ),
                        }
                    }
                }
            }
        }
    }
}

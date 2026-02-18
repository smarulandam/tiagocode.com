use dioxus::prelude::*;

use crate::Route;
use content_core::application::domain::layout::MenuTree;
use crate::views::components::common::Menu;

#[component]
pub fn Navbar(main_menu: MenuTree, social_menu: MenuTree) -> Element {
    let mut open_mobile_menu = use_signal(|| false);

    rsx! {
        nav { class: "sticky top-0 z-40 border-b border-gray-200 bg-white shadow-[0_8px_26px_0_rgba(22,24,26,0.15)] transition ease-out duration-[160ms]",
            div { class: "mx-auto flex max-w-[1320px] flex-wrap items-center justify-between px-5 py-4 xl:px-0",

                Link {
                    class: "flex items-center space-x-3",
                    to: Route::PortfolioPage {
                        lang: "en".to_string(),
                    },
                    img {
                        class: "h-8",
                        alt: "Tiagocode Logo",
                        src: asset!("/assets/images/logo_teal.svg"),
                    }
                    span { class: "hidden self-center whitespace-nowrap text-2xl font-semibold uppercase tracking-widest text-primary md:block",
                        "Tiagocode"
                    }
                }

                div { class: "flex items-center",
                    div { class: "md:order-2",
                        Menu {
                            menu: social_menu.clone(),
                            container_class: Some("flex items-center font-medium".to_string()),
                            item_class: Some("block py-0 px-3 -mr-3".to_string()),
                            anchor_class: Some(String::new()),
                        }
                    }
                    div { class: "hidden md:order-1 md:block",
                        Menu {
                            menu: main_menu.clone(),
                            container_class: Some("mr-4 flex items-center font-medium".to_string()),
                            item_class: Some("block px-3 py-0 uppercase".to_string()),
                            anchor_class: Some(
                                "text-sm font-medium tracking-[0.04em] text-black hover:text-primary".to_string(),
                            ),
                        }
                    }
                }

                div { class: "block md:hidden",
                    button {
                        r#type: "button",
                        class: "inline-flex h-10 w-10 items-center justify-center rounded-lg bg-white p-2 text-sm text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200",
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
                    Menu {
                        menu: main_menu,
                        container_class: Some(
                            "mt-4 flex flex-col rounded-lg border border-gray-100 bg-gray-50 p-4 font-medium md:mt-0 md:flex-row md:space-x-8 md:border-0 md:bg-white md:p-0 rtl:space-x-reverse"
                                .to_string(),
                        ),
                        item_class: Some(
                            "block rounded-sm px-3 py-2 text-gray-900 uppercase hover:bg-gray-100 md:border-0 md:p-0 md:hover:bg-transparent"
                                .to_string(),
                        ),
                        anchor_class: Some("block text-sm font-medium tracking-[0.04em]".to_string()),
                    }
                }
            }
        }
    }
}

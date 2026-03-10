use dioxus::prelude::*;

use crate::components::common::{Logo, NavigationMenu};
use content_core::application::domain::layout::MenuTree;

#[component]
pub fn Navbar(main_menu: MenuTree, social_menu: MenuTree) -> Element {
    let mut open_mobile_menu = use_signal(|| false);

    rsx! {
        nav {
            div { class: "flex flex-wrap items-center justify-between max-w-[1320px] mx-auto py-4 px-5 xl:px-0",
                Logo {}
                div { class: "flex items-center",
                    div { class: "md:order-2",
                        NavigationMenu {
                            items: social_menu.items().clone(),
                            item_class: "block py-0 px-3 -mr-3".to_string(),
                            container_class: "flex items-center font-medium".to_string(),
                        }
                    }
                    div { class: "md:order-1 hidden md:block",
                        NavigationMenu {
                            items: main_menu.items().clone(),
                            item_class: "block py-0 px-3 uppercase".to_string(),
                            container_class: "flex items-center font-medium mr-4".to_string(),
                        }
                    }
                }
                div { class: "block md:hidden",
                    button {
                        r#type: "button",
                        class: "inline-flex items-center justify-center w-10 h-10 rounded-lg bg-white p-2 text-sm text-gray-500 hover:bg-gray-100 focus:outline-hidden focus:ring-2 focus:ring-gray-200",
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
                    class: if open_mobile_menu() { "w-full md:hidden md:w-auto" } else { "w-full md:hidden md:w-auto hidden" },
                    NavigationMenu {
                        items: main_menu.items().clone(),
                        anchor_class: "block".to_string(),
                        item_class: "block py-2 px-3 text-gray-900 rounded-xs hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:p-0 uppercase".to_string(),
                        container_class: "font-medium flex flex-col gap-2 p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:gap-8 md:mt-0 md:border-0 md:bg-white".to_string(),
                    }
                }
            }
        }
    }
}

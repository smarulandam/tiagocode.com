use dioxus::prelude::*;

use crate::components::common::Navbar;
use content_core::application::domain::layout::MenuTree;

#[component]
pub fn SiteLayout(main_menu: MenuTree, social_menu: MenuTree, children: Element) -> Element {
    rsx! {
        header {
            class: "fixed w-full z-20 top-0 start-0 bg-white border-b border-gray-200 shadow-smoke-shadow hover:shadow-smoke-shadow-hover transition ease-out duration-[160ms]",
            id: "header",
            Navbar {
                main_menu,
                social_menu,
            }
        }
        main { class: "bg-smoke",
            div {
                class: "container max-w-[1320px] mx-auto px-5 xl:px-0 pt-[110px] lg:pt-[128px] min-h-[100vh]",
                {children}
            }
        }
        footer { class: "bg-smoke text-center py-8 text-sm text-gray-500",
            p { "Made with love by Santiago Marulanda ❤️." }
        }
    }
}

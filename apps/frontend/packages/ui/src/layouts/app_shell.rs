use dioxus::prelude::*;

use crate::components::common::Navbar;
use content_core::application::domain::layout::MenuTree;

#[component]
pub fn AppShell(main_menu: MenuTree, social_menu: MenuTree, children: Element) -> Element {
    rsx! {
        div {
            class: "relative min-h-screen text-foreground",

            Navbar {
                main_menu,
                social_menu,
            }

            main {
                class: "relative mx-auto w-full max-w-[1320px] px-5 pb-14 pt-4 sm:pt-5 lg:pt-6 xl:px-0",
                {children}
            }
        }
    }
}

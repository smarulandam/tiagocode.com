use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        ul { class: "sidebar-links space-x-4 text-center font-mono text-xs font-medium uppercase tracking-[0.11em] text-primary-foreground lg:space-x-0 lg:text-left",
            SidebarItem { title: "About Me", href: "#about" }
            SidebarItem { title: "Tech Blog", href: "#blog" }
            SidebarItem { title: "Resume", href: "#resume" }
            SidebarItem { title: "Projects", href: "#portfolio" }
        }
    }
}

#[component]
fn SidebarItem(title: &'static str, href: &'static str) -> Element {
    rsx! {
        li { class: "list-none inline-block lg:block",
            a {
                href,
                class: "section-link group relative inline-flex h-9 w-9 items-center justify-center rounded-lg border border-transparent py-3 text-primary-foreground transition duration-150 ease-linear lg:h-auto lg:w-auto lg:justify-between lg:rounded-lg lg:border-none lg:block",
                span { class: "hidden lg:inline-block", "{title}" }
                span { class: "nav-circle absolute right-0 top-1/2 hidden h-1.5 w-1.5 -translate-y-1/2 rounded-full lg:inline-block" }
            }
        }
    }
}

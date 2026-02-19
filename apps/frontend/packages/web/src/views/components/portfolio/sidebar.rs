use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        ul { class: "grid gap-1.5 space-x-4 text-center font-mono text-xs font-medium uppercase tracking-[0.11em] text-primary-foreground lg:space-x-0 lg:text-left",
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
                class: "group relative inline-flex h-9 w-9 items-center justify-center rounded-lg border border-transparent py-3 text-primary-foreground/85 transition duration-150 ease-linear hover:bg-slate-400/20 hover:text-white lg:block lg:h-auto lg:w-auto lg:justify-between lg:rounded-lg lg:border-none lg:px-2 lg:py-2",
                span { class: "hidden lg:inline-block", "{title}" }
                span { class: "absolute right-0 top-1/2 hidden h-1.5 w-1.5 -translate-y-1/2 rounded-full bg-cyan-300 shadow-[0_0_0_6px_rgba(6,182,212,0.15)] transition-all duration-150 group-hover:bg-cyan-200 group-hover:shadow-[0_0_0_6px_rgba(6,182,212,0.25)] lg:inline-block" }
            }
        }
    }
}

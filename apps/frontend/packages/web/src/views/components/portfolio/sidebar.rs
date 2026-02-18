use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        ul { class: "space-x-4 text-center font-mono text-sm font-normal uppercase tracking-wider text-primary-foreground lg:space-x-0 lg:text-left",
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
                class: "section-link group relative inline-flex h-9 w-9 items-center justify-center rounded-full border border-transparent border-dashed py-3 text-primary-foreground transition duration-100 ease-linear lg:h-auto lg:w-auto lg:justify-normal lg:rounded-none lg:border-none lg:block",
                span { class: "hidden lg:inline-block hover:text-accent", "{title}" }
                span { class: "nav-circle absolute right-0 top-1/2 hidden h-[5px] w-[5px] -translate-y-1/2 lg:inline-block before:absolute before:left-1/2 before:top-1/2 before:h-[5px] before:w-[5px] before:-translate-x-1/2 before:-translate-y-1/2 before:rounded-full before:bg-accent before:opacity-70 before:transition-all before:duration-200 before:ease-out before:content-[''] group-hover:before:opacity-100" }
            }
        }
    }
}

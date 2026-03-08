use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <ul class="flex flex-col gap-1">
            <SidebarItem title="About Me" href="#about" />
            <SidebarItem title="Tech Blog" href="#blog" />
            <SidebarItem title="Resume" href="#resume" />
            <SidebarItem title="Projects" href="#portfolio" />
        </ul>
    }
}

#[component]
fn SidebarItem(title: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <li class="list-none">
            <a href=href class="section-link sidebar-link">
                <span>{title}</span>
                <span class="sidebar-dot"></span>
            </a>
        </li>
    }
}

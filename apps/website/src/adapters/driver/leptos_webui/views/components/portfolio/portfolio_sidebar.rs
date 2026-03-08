use leptos::prelude::*;

#[component]
pub fn PortfolioSidebar() -> impl IntoView {
    view! {
        <ul class="flex flex-wrap justify-center gap-4 font-mono font-normal uppercase text-sm tracking-wider text-center lg:flex-col lg:items-start lg:gap-0 lg:text-left">
            <PortfolioSidebarLink title="About Me" href="#about" />
            <PortfolioSidebarLink title="Tech Blog" href="#blog" />
            <PortfolioSidebarLink title="Resume" href="#resume" />
            <PortfolioSidebarLink title="Projects" href="#portfolio" />
        </ul>
    }
}

#[component]
fn PortfolioSidebarLink(title: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <li class="list-none">
            <a href=href class="section-link group inline-flex justify-center items-center lg:block lg:justify-normal relative w-9 h-9 border border-transparent border-dashed rounded-full lg:w-auto lg:h-auto lg:border-none lg:rounded-none text-white py-3 transition ease-linear duration-100">
                <span class="hidden lg:inline-block hover:text-sheengold">{title}</span>
                <span class="nav-circle hidden lg:inline-block absolute top-1/2 right-0 -translate-y-1/2 w-[5px] h-[5px] before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-sheengold before:w-[5px] before:h-[5px] before:rounded-full before:opacity-70 before:transition-all before:ease-out before:duration-200 group-hover:before:opacity-100"></span>
            </a>
        </li>
    }
}

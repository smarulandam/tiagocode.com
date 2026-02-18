use dioxus::prelude::*;

#[component]
pub fn TimelineSection(icon_class: &'static str, children: Element) -> Element {
    rsx! {
        div {
            class: "relative space-y-7 pl-5 before:absolute before:top-0 before:left-0 before:h-full before:w-[1px] before:border-l before:border-black/20 before:border-dashed before:content-['']",
            div { class: "text-3xl leading-none", i { class: icon_class } }
            {children}
        }
    }
}

#[component]
pub fn TimelineSectionItem(date: String, title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "group",
            div {
                class: "relative inline-block rounded-full border border-black/20 border-dashed px-4 py-2 font-mono text-sm font-medium uppercase tracking-[0.5px] text-zeus transition duration-100 ease-linear before:absolute before:top-1/2 before:left-[-20px] before:h-[1px] before:w-[20px] before:border-t before:border-black/20 before:border-dashed before:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:h-[5px] after:w-[5px] after:-translate-y-1/2 after:rounded-full after:bg-black after:content-[''] group-hover:text-black",
                "{date}"
            }
            h3 {
                class: "mb-1 mt-2 font-display text-lg font-medium lg:mb-2 lg:mt-3 lg:text-xl",
                "{title}"
            }
            span { class: "text-zeus", "{subtitle}" }
        }
    }
}

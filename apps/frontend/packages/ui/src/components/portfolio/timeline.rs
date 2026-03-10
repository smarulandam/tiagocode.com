use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelineIcon {
    Education,
    Experience,
}

#[component]
pub fn TimelineGroup(children: Element, icon: TimelineIcon) -> Element {
    rsx! {
        div {
            class: "relative flex flex-col gap-7 pl-5 before:content-[''] before:absolute before:top-0 before:left-0 before:w-[1px] before:h-full before:border-l before:border-black/20 before:border-dashed",
            div { class: "text-3xl",
                TimelineGroupIcon { icon }
            }
            {children}
        }
    }
}

#[component]
fn TimelineGroupIcon(icon: TimelineIcon) -> Element {
    match icon {
        TimelineIcon::Education => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "h-8 w-8",
                path { d: "M22 10L12 5 2 10l10 5 10-5z" }
                path { d: "M6 12.5V16.5C6 18.4 8.7 20 12 20C15.3 20 18 18.4 18 16.5V12.5" }
            }
        },
        TimelineIcon::Experience => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "h-8 w-8",
                rect {
                    x: "3",
                    y: "7",
                    width: "18",
                    height: "13",
                    rx: "2",
                }
                path { d: "M8 7V5C8 3.9 8.9 3 10 3H14C15.1 3 16 3.9 16 5V7" }
                path { d: "M3 12H21" }
            }
        },
    }
}

#[component]
pub fn TimelineEntry(date: String, title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "group",
            div {
                class: "relative inline-block px-4 py-2 rounded-full border border-black/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-zeus dark:text-white/70 group-hover:text-black transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full",
                "{date}"
            }
            h3 { class: "font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2",
                "{title}"
            }
            span { class: "text-zeus dark:text-white/70", "{subtitle}" }
        }
    }
}

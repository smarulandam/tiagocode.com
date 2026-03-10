use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SocialIconKind {
    Github,
    Youtube,
    Linkedin,
}

#[component]
pub fn EducationIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "h-8 w-8".to_string());

    rsx! {
        svg {
            class,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.8",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M22 10L12 5 2 10l10 5 10-5z" }
            path { d: "M6 12.5V16.5C6 18.4 8.7 20 12 20C15.3 20 18 18.4 18 16.5V12.5" }
        }
    }
}

#[component]
pub fn ExperienceIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "h-8 w-8".to_string());

    rsx! {
        svg {
            class,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.8",
            stroke_linecap: "round",
            stroke_linejoin: "round",
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
    }
}

#[component]
pub fn DownloadIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "h-4 w-4".to_string());

    rsx! {
        svg {
            class,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 3V14" }
            path { d: "M7 10L12 15L17 10" }
            path { d: "M4 20H20" }
        }
    }
}

#[component]
pub fn SocialIcon(icon: SocialIconKind, class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "h-5 w-5".to_string());

    match icon {
        SocialIconKind::Github => rsx! {
            svg {
                class,
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path { d: "M12 2C6.5 2 2 6.6 2 12.2C2 16.7 4.9 20.5 8.8 21.8C9.3 21.9 9.5 21.6 9.5 21.3V19.6C6.7 20.2 6.1 18.4 6.1 18.4C5.6 17 4.9 16.6 4.9 16.6C3.8 15.8 5 15.8 5 15.8C6.2 15.9 6.8 17.1 6.8 17.1C7.9 19 9.8 18.5 10.5 18.2C10.6 17.4 10.9 16.9 11.3 16.6C9.1 16.4 6.9 15.5 6.9 11.6C6.9 10.5 7.3 9.6 8 8.9C7.9 8.6 7.6 7.5 8.1 6.1C8.1 6.1 9 5.8 11.3 7.3C12.1 7.1 12.9 7 13.7 7C14.5 7 15.3 7.1 16.1 7.3C18.4 5.8 19.3 6.1 19.3 6.1C19.8 7.5 19.5 8.6 19.4 8.9C20.1 9.6 20.5 10.5 20.5 11.6C20.5 15.5 18.3 16.4 16.1 16.6C16.6 17 16.9 17.7 16.9 18.8V21.3C16.9 21.6 17.1 22 17.7 21.8C21.6 20.5 24.5 16.7 24.5 12.2C24.5 6.6 20 2 14.5 2H12Z" }
            }
        },
        SocialIconKind::Youtube => rsx! {
            svg {
                class,
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path { d: "M23.5 7.3C23.2 6.1 22.3 5.2 21.1 4.9C18.9 4.3 12 4.3 12 4.3C12 4.3 5.1 4.3 2.9 4.9C1.7 5.2 0.8 6.1 0.5 7.3C0 9.5 0 12 0 12C0 12 0 14.5 0.5 16.7C0.8 17.9 1.7 18.8 2.9 19.1C5.1 19.7 12 19.7 12 19.7C12 19.7 18.9 19.7 21.1 19.1C22.3 18.8 23.2 17.9 23.5 16.7C24 14.5 24 12 24 12C24 12 24 9.5 23.5 7.3ZM9.7 15.6V8.4L16 12L9.7 15.6Z" }
            }
        },
        SocialIconKind::Linkedin => rsx! {
            svg {
                class,
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path { d: "M20.4 20.4H16.8V14.9C16.8 13.6 16.8 11.9 15 11.9C13.2 11.9 12.9 13.3 12.9 14.8V20.4H9.3V9H12.8V10.6H12.9C13.4 9.7 14.6 8.8 16.4 8.8C20.1 8.8 20.4 11.2 20.4 14.3V20.4ZM5.2 7.4C4 7.4 3 6.4 3 5.2C3 4 4 3 5.2 3C6.4 3 7.4 4 7.4 5.2C7.4 6.4 6.4 7.4 5.2 7.4ZM7 20.4H3.4V9H7V20.4Z" }
            }
        },
    }
}

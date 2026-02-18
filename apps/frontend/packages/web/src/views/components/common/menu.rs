use dioxus::prelude::*;

use content_core::application::domain::layout::{MenuItem, MenuTree};

#[component]
pub fn Menu(
    menu: MenuTree,
    container_class: Option<String>,
    item_class: Option<String>,
    anchor_class: Option<String>,
) -> Element {
    let item_class = item_class.unwrap_or_default();
    let anchor_class = anchor_class.unwrap_or_default();

    rsx! {
        ul {
            class: container_class.unwrap_or_default(),
            for item in menu.items().iter() {
                MenuLink {
                    item: item.clone(),
                    item_class: item_class.clone(),
                    anchor_class: anchor_class.clone(),
                }
            }
        }
    }
}

#[component]
fn MenuLink(item: MenuItem, item_class: String, anchor_class: String) -> Element {
    let title = item.title().to_string();
    let href = item.url().to_string();
    let is_external = item.url().is_absolute();
    let fallback_icon_class = social_icon_class(&title, &href);

    rsx! {
        li {
            class: item_class,
            a {
                class: if item.icon().is_some() {
                    anchor_class.clone()
                } else {
                    format!("{} hover:text-[#86a873] w-full", anchor_class.clone()).trim().to_string()
                },
                href: href,
                target: if is_external { "_blank" } else { "_self" },
                rel: if is_external { "noopener noreferrer" } else { "" },
                title: title.clone(),
                aria_label: title.clone(),

                if let Some(icon) = item.icon().clone() {
                    img {
                        class: "h-8",
                        src: icon.url().to_string(),
                        alt: icon.alt().to_string(),
                        loading: "lazy",
                    }
                } else if let Some(icon_class) = fallback_icon_class {
                    i {
                        class: format!("bi {icon_class} text-[1.35rem] leading-none text-black"),
                        aria_hidden: "true",
                    }
                } else {
                    span { class: "h-8", "{title}" }
                }
            }
        }
    }
}

fn social_icon_class(title: &str, url: &str) -> Option<&'static str> {
    let key = format!("{} {}", title.to_lowercase(), url.to_lowercase());

    if key.contains("github") {
        Some("bi-github")
    } else if key.contains("youtube") || key.contains("youtu.be") {
        Some("bi-youtube")
    } else if key.contains("linkedin") {
        Some("bi-linkedin")
    } else {
        None
    }
}

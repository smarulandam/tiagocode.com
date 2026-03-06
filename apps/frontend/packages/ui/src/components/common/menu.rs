use dioxus::prelude::*;

use content_core::application::domain::layout::{MenuItem, MenuTree};

use super::{SocialIcon, SocialIconKind};

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
        ul { class: container_class.unwrap_or_default(),
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
    let fallback_icon = social_icon_kind(&title, &href);

    rsx! {
        li { class: item_class,
            a {
                class: if item.icon().is_some() || fallback_icon.is_some() {
                    anchor_class.clone()
                } else {
                    format!("{} w-full", anchor_class.clone()).trim().to_string()
                },
                href,
                target: if is_external { "_blank" } else { "_self" },
                rel: if is_external { "noopener noreferrer" } else { "" },
                title: title.clone(),
                aria_label: title.clone(),

                if let Some(icon) = item.icon().clone() {
                    img {
                        class: "h-2 object-contain",
                        src: icon.url().to_string(),
                        alt: icon.alt().to_string(),
                        loading: "lazy",
                    }
                } else if let Some(icon) = fallback_icon {
                    SocialIcon {
                        icon,
                        class: Some("h-7 w-7 text-current".to_string()),
                    }
                } else {
                    "{title}"
                }
            }
        }
    }
}

fn social_icon_kind(title: &str, url: &str) -> Option<SocialIconKind> {
    let key = format!("{} {}", title.to_lowercase(), url.to_lowercase());

    if key.contains("github") {
        Some(SocialIconKind::Github)
    } else if key.contains("youtube") || key.contains("youtu.be") {
        Some(SocialIconKind::Youtube)
    } else if key.contains("linkedin") {
        Some(SocialIconKind::Linkedin)
    } else {
        None
    }
}

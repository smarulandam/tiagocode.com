use dioxus::prelude::*;

use content_core::application::domain::common::Image;
use content_core::application::domain::layout::MenuItem as LayoutMenuItem;

#[component]
pub fn NavigationMenu(
    items: Vec<LayoutMenuItem>,
    #[props(default = "".to_string())] item_class: String,
    #[props(default = "".to_string())] container_class: String,
    #[props(default = "".to_string())] anchor_class: String,
) -> Element {
    rsx! {
        ul { class: container_class,
            for item in items {
                NavigationMenuItem {
                    url: item.url().to_string(),
                    icon: item.icon().clone(),
                    title: item.title().to_string(),
                    class: item_class.clone(),
                    anchor_class: anchor_class.clone(),
                    is_external: item.url().is_absolute(),
                }
            }
        }
    }
}

#[component]
fn NavigationMenuItem(
    url: String,
    title: String,
    class: String,
    anchor_class: String,
    icon: Option<Image>,
    #[props(default = false)] is_external: bool,
) -> Element {
    let target = if is_external { "_blank" } else { "_self" };

    rsx! {
        li { class: class.clone(),
            match icon {
                Some(icon) => rsx! {
                    a {
                        target,
                        href: url.clone(),
                        title: title.clone(),
                        class: anchor_class.clone(),
                        img {
                            src: icon.url().to_string(),
                            alt: icon.alt().to_string(),
                            class: "h-8",
                        }
                    }
                },
                None => rsx! {
                    a {
                        target,
                        href: url,
                        title: title.clone(),
                        class: format!(
                            "{} hover:text-asparagus w-full",
                            anchor_class.clone()
                        ),
                        span { class: "h-8", "{title}" }
                    }
                },
            }
        }
    }
}

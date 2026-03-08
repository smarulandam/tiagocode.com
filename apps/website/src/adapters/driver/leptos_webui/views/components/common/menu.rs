use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::application::domain::common::Image;
use crate::application::domain::layout::MenuItem;

#[component]
pub fn Menu(
    items: Vec<MenuItem>,
    #[prop(default = "")] item_class: &'static str,
    #[prop(default = "")] container_class: &'static str,
    #[prop(default = "")] anchor_class: &'static str,
) -> impl IntoView {
    view! {
        <ul class=container_class>
            {items
                .into_iter()
                .map(|item| {
                    view! {
                        <MenuItem
                            url=item.url().clone().to_string()
                            icon=item.icon().clone()
                            title=item.title().to_string()
                            class=item_class.to_string()
                            anchor_class=anchor_class.to_string()
                            is_external=item.url().is_absolute()
                        />
                    }
                })
                .collect_view()}
        </ul>
    }
}

#[component]
pub fn MenuItem(
    url: String,
    title: String,
    class: String,
    anchor_class: String,
    icon: Option<Image>,
    #[prop(default = false)] is_external: bool,
) -> impl IntoView {
    let location = use_location();
    let active_url = url.clone();
    let active_prefix = format!("{}/", active_url.clone());
    let is_external_link = is_external || active_url.starts_with("mailto:");
    let is_hash_link = active_url.starts_with('#');
    let target = if is_external_link { "_blank" } else { "_self" };
    let is_active = Signal::derive(move || {
        if is_external_link || is_hash_link {
            return false;
        }

        let pathname = location.pathname.read().to_string();
        pathname == active_url
            || (active_url.ends_with("/articles") && pathname.starts_with(active_prefix.as_str()))
    });
    let classes = move || {
        let mut classes = anchor_class.clone();
        if is_active.get() {
            classes.push_str(" is-active");
        }
        classes
    };

    view! {
        <li class=class.clone()>
            {match icon.clone() {
                Some(icon) => view! {
                    <a
                        target=target
                        href=url.to_string()
                        title=title.to_string()
                        class=classes()
                        aria-current=move || if is_active.get() { "page" } else { "" }
                    >
                        <img src=icon.url().to_string() alt=icon.alt().to_string() />
                    </a>
                }.into_any(),
                None => view! {
                    <a
                        target=target
                        href=url.to_string()
                        title=title.to_string()
                        class=classes()
                        aria-current=move || if is_active.get() { "page" } else { "" }
                    >
                        <span>{title.to_string()}</span>
                    </a>
                }.into_any()
            }}
        </li>
    }
}

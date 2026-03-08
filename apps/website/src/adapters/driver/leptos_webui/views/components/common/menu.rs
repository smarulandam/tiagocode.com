use leptos::prelude::*;

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
    let target = if is_external { "_blank" } else { "_self" };

    view! {
        <li class=class.clone()>
            {match icon.clone() {
                Some(icon) => view! {
                    <a target=target href=url.to_string() title=title.to_string() class=anchor_class.clone()>
                        <img src=icon.url().to_string() alt=icon.alt().to_string() class="h-8" />
                    </a>
                }.into_any(),
                None => view! {
                    <a target=target href=url.to_string() title=title.to_string() class=format!("{} hover:text-asparagus w-full", anchor_class.clone())>
                        <span class="h-8">{title.to_string()}</span>
                    </a>
                }.into_any()
            }}
        </li>
    }
}

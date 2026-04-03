use leptos::prelude::*;
use leptos_router::components::A;

use crate::application::domain::common::Image;
use crate::application::domain::layout::MenuItem as LayoutMenuItem;

#[component]
pub fn NavigationMenu(
    items: Vec<LayoutMenuItem>,
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
                        <NavigationMenuItem
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
fn NavigationMenuItem(
    url: String,
    title: String,
    class: String,
    anchor_class: String,
    icon: Option<Image>,
    #[prop(default = false)] is_external: bool,
) -> impl IntoView {
    let is_router_link = !is_external && url.starts_with('/');

    view! {
        <li class=class.clone()>
            {match icon.clone() {
                Some(icon) if is_router_link => view! {
                    <A href=url.clone() attr:title=title.clone() attr:class=anchor_class.clone()>
                        <img src=icon.url().to_string() alt=icon.alt().to_string() class="h-8" />
                    </A>
                }.into_any(),
                Some(icon) if is_external => view! {
                    <a target="_blank" href=url.to_string() title=title.to_string() class=anchor_class.clone()>
                        <img src=icon.url().to_string() alt=icon.alt().to_string() class="h-8" />
                    </a>
                }.into_any(),
                Some(icon) => view! {
                    <a href=url.to_string() title=title.to_string() class=anchor_class.clone()>
                        <img src=icon.url().to_string() alt=icon.alt().to_string() class="h-8" />
                    </a>
                }.into_any(),
                None if is_router_link => {
                    let link_title = title.clone();
                    let link_label = title.clone();
                    let link_class = format!("{} hover:text-asparagus w-full", anchor_class.clone());

                    view! {
                        <A href=url.clone() attr:title=link_title attr:class=link_class>
                            <span class="h-8">{link_label}</span>
                        </A>
                    }.into_any()
                },
                None if is_external => view! {
                    <a target="_blank" href=url.to_string() title=title.to_string() class=format!("{} hover:text-asparagus w-full", anchor_class.clone())>
                        <span class="h-8">{title.to_string()}</span>
                    </a>
                }.into_any()
                ,
                None => view! {
                    <a href=url.to_string() title=title.to_string() class=format!("{} hover:text-asparagus w-full", anchor_class.clone())>
                        <span class="h-8">{title.to_string()}</span>
                    </a>
                }.into_any()
            }}
        </li>
    }
}

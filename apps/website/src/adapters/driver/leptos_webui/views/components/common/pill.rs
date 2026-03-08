use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn Pill(
    text: String,
    #[prop(default = "")] class: &'static str,
    #[prop(optional)] emoji: String,
    #[prop(optional, default = String::new())] link: String,
) -> impl IntoView {
    let location = use_location();
    let has_link = !link.is_empty();
    let link_href = link.clone();
    let active_link = link.clone();
    let is_active = Signal::derive(move || {
        !active_link.is_empty() && location.pathname.read().as_str() == active_link
    });

    let content = view! {
        <span class:hidden=move || emoji.is_empty()>{emoji.clone()}</span>
        <span>{text}</span>
    };

    let classes = move || {
        let mut classes = String::from("tag-chip");
        if !has_link {
            classes.push_str(" tag-chip--static");
        }
        if is_active.get() {
            classes.push_str(" is-active");
        }
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        {if !has_link {
            view! {
                <span class=classes()>
                    {content}
                </span>
            }.into_any()
        } else {
            view! {
                <a
                    href=link_href
                    target="_self"
                    class=classes()
                    aria-current=move || if is_active.get() { "page" } else { "" }
                >
                    {content}
                </a>
            }.into_any()
        }}
    }
}

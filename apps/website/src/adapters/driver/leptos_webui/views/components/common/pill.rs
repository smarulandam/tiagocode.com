use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Pill(
    text: String,
    #[prop(default = "")] class: &'static str,
    #[prop(optional)] emoji: String,
    #[prop(optional, default = String::new())] link: String,
) -> impl IntoView {
    let content = view! {
        <span class="mr-2" class:hidden=move || emoji.is_empty()>{emoji.clone()}</span>
        <span class="inline-block font-mono text-sm">
            {text}
        </span>
    };

    view! {
        <div class=format!("inline-block px-4 py-2 me-2 rounded-full border border-black/20 border-dashed text-zeus hover:bg-sheengold/70 transition ease-linear duration-100 {class}")>
            {if link.is_empty() {
                content.into_any()
            } else if link.starts_with('/') {
                view! { <A href=link>{content}</A> }.into_any()
            } else {
                view! { <a href=link>{content}</a> }.into_any()
            }}
        </div>
    }
}

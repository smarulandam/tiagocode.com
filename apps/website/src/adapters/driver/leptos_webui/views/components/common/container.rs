use leptos::prelude::*;

const CONTAINER_CLASS: &'static str = "section-panel surface-panel";

#[component]
pub fn Container(
    children: Children,
    #[prop(optional)] id: &'static str,
    #[prop(optional, default = CONTAINER_CLASS)] class: &'static str,
) -> impl IntoView {
    view! {
        <div id=id class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn PrimaryTitle(text: String) -> impl IntoView {
    view! { <h1 class="section-title">{text}</h1> }
}

#[component]
pub fn SecondaryTitle(text: String) -> impl IntoView {
    view! { <h2 class="section-title">{text}</h2> }
}

#[component]
pub fn Decoration(text: String) -> impl IntoView {
    view! { <span class="section-kicker">{text}</span> }
}

#[component]
pub fn Description(text: String) -> impl IntoView {
    view! { <p class="section-description">{text}</p> }
}

use leptos::prelude::*;

const CONTAINER_CLASS: &'static str = "section bg-white px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-smoke-shadow hover:shadow-smoke-shadowHover transition ease-out duration-[160ms] rounded-lg";

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
    view! { <h1 class="text-4xl font-poppins font-semibold mb-2 text-teal">{text}</h1> }
}

#[component]
pub fn SecondaryTitle(text: String) -> impl IntoView {
    view! { <h2 class="text-4xl font-poppins font-semibold mb-2 text-teal">{text}</h2> }
}

#[component]
pub fn Decoration(text: String) -> impl IntoView {
    view! {
        <span class="font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 before:content-['//'] before:pr-2 text-sheengold">
            {text}
        </span>
    }
}

#[component]
pub fn Description(text: String) -> impl IntoView {
    view! { <p class="text-zeus">{text}</p> }
}

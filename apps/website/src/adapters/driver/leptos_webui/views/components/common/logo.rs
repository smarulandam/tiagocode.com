use leptos::prelude::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <a href="/" target="_self" class="flex items-center space-x-3">
            <img src="/assets/images/logo_teal.svg" class="h-8" alt="Tiagocode Logo" />
            <span class="hidden md:block self-center text-2xl font-semibold whitespace-nowrap uppercase tracking-widest text-teal   ">
                Tiagocode
            </span>
        </a>
    }
}

use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <A href="/en" attr:class="flex items-center gap-3">
            <img src="/assets/images/logo_teal.svg" class="h-8" alt="Tiagocode Logo" />
            <span class="hidden md:block self-center text-2xl font-semibold whitespace-nowrap uppercase tracking-widest text-teal   ">
                Tiagocode
            </span>
        </A>
    }
}

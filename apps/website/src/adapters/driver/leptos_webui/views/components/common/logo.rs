use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[component]
pub fn Logo() -> impl IntoView {
    let language_home = use_params_map()
        .get_untracked()
        .get("lang")
        .map(|lang| if lang == "es" { "/es" } else { "/" })
        .unwrap_or("/");

    view! {
        <A href=language_home attr:class="flex items-center gap-3">
            <img src="/assets/images/logo_teal.svg" class="h-8" alt="Tiagocode Logo" />
            <span class="hidden md:block self-center text-2xl font-semibold whitespace-nowrap uppercase tracking-widest text-teal   ">
                Tiagocode
            </span>
        </A>
    }
}

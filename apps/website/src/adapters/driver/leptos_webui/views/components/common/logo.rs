use leptos::prelude::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <a href="/" target="_self" class="brand-lockup">
            <img src="/assets/images/logo_teal.svg" class="brand-mark" alt="Tiagocode Logo" />
            <span class="brand-wordmark">
                Tiagocode
            </span>
        </a>
    }
}

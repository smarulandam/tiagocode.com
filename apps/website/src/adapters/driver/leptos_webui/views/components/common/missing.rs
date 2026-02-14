use leptos::prelude::*;

#[component]
pub fn MissingSection() -> impl IntoView {
    view! {
        <p class="text-center text-2xl font-poppins font-medium text-zeus dark:text-white/70">
            "Missing section"
        </p>
    }
}

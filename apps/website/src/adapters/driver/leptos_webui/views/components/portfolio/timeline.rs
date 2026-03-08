use leptos::prelude::*;

#[component]
pub fn Timeline(children: Children, icon: &'static str) -> impl IntoView {
    view! {
        <div class="timeline-shell">
            <div class="timeline-icon">
                <i class=icon></i>
            </div>
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineItem(date: String, title: String, subtitle: String) -> impl IntoView {
    view! {
        <div class="timeline-item">
            <div class="timeline-date">
                {date.clone()}
            </div>
            <h3 class="mt-3 font-poppins text-lg font-medium leading-tight text-zeus lg:text-xl">
                {title.clone()}
            </h3>
            <p class="mt-2 leading-relaxed text-[color:var(--color-copy-muted)]">{subtitle.clone()}</p>
        </div>
    }
}

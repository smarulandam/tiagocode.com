use leptos::prelude::*;

#[component]
pub fn TimelineGroup(children: Children, icon: &'static str) -> impl IntoView {
    let icon = if icon == "briefcase" {
        view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.8"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="h-8 w-8"
                aria-hidden="true"
            >
                <path d="M9 6V4.75A1.75 1.75 0 0 1 10.75 3h2.5A1.75 1.75 0 0 1 15 4.75V6" />
                <path d="M4.75 6h14.5A1.75 1.75 0 0 1 21 7.75v8.5A1.75 1.75 0 0 1 19.25 18H4.75A1.75 1.75 0 0 1 3 16.25v-8.5A1.75 1.75 0 0 1 4.75 6Z" />
                <path d="M3 11.5h18" />
            </svg>
        }
        .into_any()
    } else {
        view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.8"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="h-8 w-8"
                aria-hidden="true"
            >
                <path d="m3 8.5 9-4.5 9 4.5-9 4.5-9-4.5Z" />
                <path d="M7 10.5V15c0 .55.32 1.05.82 1.27l3.36 1.52c.53.24 1.12.24 1.65 0l3.35-1.52A1.4 1.4 0 0 0 17 15v-4.5" />
                <path d="M21 8.5V13" />
            </svg>
        }
        .into_any()
    };

    view! {
        <div class="relative flex flex-col gap-7 pl-5 before:content-[''] before:absolute before:top-0 before:left-0 before:w-[1px] before:h-full before:border-l before:border-black/20 before:border-dashed">
            <div class="text-deepsea">
                {icon}
            </div>
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineEntry(date: String, title: String, subtitle: String) -> impl IntoView {
    view! {
        <div class="group">
            <div class="relative inline-block px-4 py-2 rounded-full border border-black/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-zeus dark:text-white/70 group-hover:text-black transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full">
                {date.clone()}
            </div>
            <h3 class="font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2">
                {title.clone()}
            </h3>
            <span class="text-zeus dark:text-white/70">{subtitle.clone()}</span>
        </div>
    }
}

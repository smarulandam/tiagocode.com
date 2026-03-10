use dioxus::prelude::*;

#[component]
pub fn LanguageSelector(
    #[props(default)] container_class: String,
    #[props(default)] button_class: String,
    #[props(default)] menu_class: String,
) -> Element {
    let mut open_menu = use_signal(|| false);

    rsx! {
        div {
            class: if container_class.is_empty() {
                "relative"
            } else {
                container_class.as_str()
            },

            button {
                r#type: "button",
                class: if button_class.is_empty() {
                    "inline-flex h-11 w-11 cursor-pointer items-center justify-center "
                } else {
                    button_class.as_str()
                },
                aria_label: "Language menu",
                aria_haspopup: "menu",
                aria_expanded: open_menu().to_string(),
                onclick: move |_| open_menu.set(!open_menu()),
                svg {
                    class: "h-[1.15rem] w-[1.15rem]",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.8",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "12", cy: "12", r: "9" }
                    path { d: "M3 12H21" }
                    path { d: "M12 3C14.8 5.7 16.4 8.8 16.4 12C16.4 15.2 14.8 18.3 12 21" }
                    path { d: "M12 3C9.2 5.7 7.6 8.8 7.6 12C7.6 15.2 9.2 18.3 12 21" }
                }
                span { class: "sr-only", "Open language options" }
            }

            if open_menu() {
                div {
                    class: if menu_class.is_empty() {
                        "absolute right-0 top-[calc(100%+0.55rem)] z-50 min-w-[11rem] rounded-xl border border-border/80 bg-white/95 p-1.5 shadow-[0_16px_28px_-20px_rgba(8,15,30,0.85)] backdrop-blur"
                    } else {
                        menu_class.as_str()
                    },
                    role: "menu",
                    button {
                        r#type: "button",
                        class: "flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left font-mono text-[0.82rem] font-semibold uppercase tracking-[0.06em] text-foreground/90 transition duration-150 ease-out hover:bg-accent/12 hover:text-primary",
                        role: "menuitem",
                        onclick: move |_| open_menu.set(false),
                        "🇺🇸 English"
                    }
                    button {
                        r#type: "button",
                        class: "flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left font-mono text-[0.82rem] font-semibold uppercase tracking-[0.06em] text-foreground/90 transition duration-150 ease-out hover:bg-accent/12 hover:text-primary",
                        role: "menuitem",
                        onclick: move |_| open_menu.set(false),
                        "🇪🇸 Español"
                    }
                }
            }
        }
    }
}

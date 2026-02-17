use dioxus::prelude::*;

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        section {
            class: "mx-auto w-full max-w-5xl rounded-3xl border border-slate-200/70 bg-white/90 p-7 shadow-[0_20px_50px_-30px_rgba(15,23,42,0.5)] backdrop-blur sm:p-8",
            div {
                class: "flex items-start justify-between gap-5",
                div {
                    h4 {
                        class: "font-display text-xl font-semibold text-slate-900",
                        "Server Echo Playground"
                    }
                    p {
                        class: "mt-1 text-sm text-slate-600",
                        "Type and test server round-trips in real time."
                    }
                }
                span {
                    class: "rounded-full border border-emerald-200 bg-emerald-50 px-2.5 py-1 text-xs font-semibold text-emerald-700",
                    "Live"
                }
            }
            div { class: "mt-6",
                input {
                    class: "w-full rounded-xl border border-slate-300 bg-white px-3 py-2.5 text-slate-900 shadow-sm outline-none transition placeholder:text-slate-400 focus:border-sky-500 focus:ring-4 focus:ring-sky-100",
                    placeholder: "Type here to echo...",
                    oninput:  move |event| async move {
                        let data = api::echo(event.value()).await.unwrap();
                        response.set(data);
                    },
                }
            }

            if !response().is_empty() {
                div {
                    class: "mt-6 rounded-xl border border-sky-200 bg-sky-50 px-4 py-3.5",
                    p {
                        class: "text-xs font-semibold uppercase tracking-wide text-sky-700",
                        "Server response"
                    }
                    p {
                        class: "mt-1 text-sm text-slate-800",
                        i { class: "font-medium", "{response}" }
                    }
                }
            }
        }
    }
}

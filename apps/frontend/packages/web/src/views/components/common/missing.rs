use dioxus::prelude::*;

#[component]
pub fn MissingSection() -> Element {
    rsx! {
        div { class: "rounded-2xl border border-dashed border-border/80 bg-surface-soft/60 px-4 py-3 text-sm text-muted-foreground",
            "Section unavailable"
        }
    }
}

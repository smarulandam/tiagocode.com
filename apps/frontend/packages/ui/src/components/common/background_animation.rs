use dioxus::prelude::*;

#[component]
pub fn BackgroundAnimation() -> Element {
    rsx! {
        ul { class: "circles",
            for _ in 0..16 {
                li {}
            }
        }
    }
}

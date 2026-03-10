use dioxus::prelude::*;

#[component]
pub fn Pill(
    text: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "".to_string())] emoji: String,
    #[props(default = String::new())] link: String,
) -> Element {
    let wrapper_class = format!(
        "inline-block px-4 py-2 me-2 rounded-full border border-black/20 border-dashed text-zeus hover:bg-sheengold/70 transition ease-linear duration-100 {}",
        class
    );

    rsx! {
        div { class: wrapper_class,
            if link.is_empty() {
                span {
                    class: if emoji.is_empty() { "hidden mr-2" } else { "mr-2" },
                    "{emoji}"
                }
                span { class: "inline-block font-mono text-sm", "{text}" }
            } else {
                a { href: link, target: "_self",
                    span {
                        class: if emoji.is_empty() { "hidden mr-2" } else { "mr-2" },
                        "{emoji}"
                    }
                    span { class: "inline-block font-mono text-sm", "{text}" }
                }
            }
        }
    }
}

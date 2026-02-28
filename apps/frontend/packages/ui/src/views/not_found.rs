use dioxus::prelude::*;

use crate::components::common::NotFoundError;

#[component]
pub fn NotFoundView(route: Option<String>) -> Element {
    rsx! {
        NotFoundError { route }
    }
}

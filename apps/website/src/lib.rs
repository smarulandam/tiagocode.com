#![recursion_limit = "512"]
#![feature(random)]

pub mod adapters;
pub mod application;
pub mod helpers;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use adapters::driver::leptos_webui::views::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

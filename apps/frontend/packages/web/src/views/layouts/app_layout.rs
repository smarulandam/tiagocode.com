use dioxus::prelude::*;

use crate::views::components::common::{Navbar, NotFoundError, UnexpectedError};
use crate::Route;

#[component]
pub fn AppLayout() -> Element {
    let layout = use_loader(|| async move { api::layout_controller().await })?;
    let layout = layout.read().clone();

    rsx! {
        ErrorBoundary {
            handle_error: move |error: ErrorContext| {
                let http_error =
                    dioxus_fullstack::FullstackContext::commit_error_status(error.error().unwrap());

                match http_error.status {
                    StatusCode::NOT_FOUND => rsx! { NotFoundError {} },
                    _ => rsx! { UnexpectedError {} },
                }
            },

            div {
                class: "app-shell",
                Navbar {
                    main_menu: layout.main_menu().clone(),
                    social_menu: layout.social_menu().clone(),
                }

                main {
                    class: "relative mx-auto w-full max-w-7xl px-4 pb-14 pt-7 sm:px-6 sm:pt-9 lg:px-8 lg:pt-10",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

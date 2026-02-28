use dioxus::prelude::*;

use crate::Route;
use ui::{AppShell, NotFoundError, UnexpectedError};

#[component]
pub fn AppLayout() -> Element {
    let layout = use_loader(|| async move { api::layout_controller().await })?;
    let layout = layout.read().clone();

    rsx! {
        ErrorBoundary {
            handle_error: move |error: ErrorContext| {
                let http_error = dioxus_fullstack::FullstackContext::commit_error_status(
                    error.error().unwrap(),
                );

                match http_error.status {
                    StatusCode::NOT_FOUND => rsx! {
                        NotFoundError { route: None }
                    },
                    _ => rsx! {
                        UnexpectedError {}
                    },
                }
            },

            AppShell {
                main_menu: layout.main_menu().clone(),
                social_menu: layout.social_menu().clone(),
                Outlet::<Route> {}
            }
        }
    }
}

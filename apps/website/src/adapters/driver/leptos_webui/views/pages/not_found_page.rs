use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::adapters::driver::leptos_webui::views::components::common::NotFoundError;
use crate::adapters::driver::leptos_webui::views::layouts::*;

/// 404 - Not Found
#[component]
pub fn NotFoundPage() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <BasicLayout>
            <Suspense fallback=move || { view! { <div>"Loading..."</div> } }>
                <div class="justify-center space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                    <NotFoundError />
                </div>
            </Suspense>
        </BasicLayout>
    }
}

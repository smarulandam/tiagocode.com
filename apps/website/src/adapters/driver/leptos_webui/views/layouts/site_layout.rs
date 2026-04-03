use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::adapters::driver::leptos_webui::controllers::layout_controller;
use crate::adapters::driver::leptos_webui::views::components::common::Navbar;

#[component]
pub fn SiteLayout() -> impl IntoView {
    view! {
        <header class="fixed w-full z-20 top-0 start-0 bg-white border-b border-gray-200 shadow-smoke-shadow hover:shadow-smoke-shadow-hover transition ease-out duration-[160ms]" id="header">
            <Suspense fallback=move || view! { <div class="h-[72px]"></div> }>
                <Await future=layout_controller() let:data>
                    {match data {
                        Ok(data) => view! {
                            <Navbar
                                main_menu=data.main_menu().clone()
                                social_menu=data.social_menu().clone()
                            />
                        }.into_any(),
                        Err(_) => view! { <span></span> }.into_any(),
                    }}
                </Await>
            </Suspense>
        </header>
        <main class="bg-smoke">
            <div class="container max-w-[1320px] mx-auto px-5 xl:px-0 pt-[110px] lg:pt-[128px] min-h-[100vh]">
                <Outlet/>
            </div>
        </main>
        <footer class="bg-smoke text-center py-8 text-sm text-gray-500">
            <p>"Made with love by Santiago Marulanda ❤️."</p>
        </footer>
    }
}

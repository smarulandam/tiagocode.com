use leptos::either::Either;
use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::controllers::layout_controller;
use crate::adapters::driver::leptos_webui::views::components::common::Navbar;

#[component]
pub fn BasicLayout(children: Children) -> impl IntoView {
    let layout = OnceResource::new(layout_controller());

    view! {
        <Transition fallback=move || { view! { <div class="bg-white"></div> }}>
            <header class="fixed w-full z-20 top-0 start-0 bg-white border-b border-gray-200 shadow-smoke-shadow hover:shadow-smoke-shadowHover transition ease-out duration-[160ms]" id="header">
                {move || {
                    layout
                    .get_untracked()
                    .map(|data| {
                        match data {
                            Err(_) => Either::Left(view! { <span></span> }),
                            Ok(data) => Either::Right(
                                view! {
                                    <Navbar
                                        main_menu=data.main_menu().clone()
                                        social_menu=data.social_menu().clone()
                                    />
                                },
                            ),
                        }
                    })
                }}
            </header>
            <main class="bg-smoke">
                <div class="container max-w-[1320px] mx-auto px-5 xl:px-0 pt-[110px] lg:pt-[128px] min-h-[100vh]">
                    {children()}
                </div>
            </main>
            <footer class="bg-smoke text-center py-8 text-sm text-gray-500">
                <p>"Made with love by Santiago Marulanda ❤️."</p>
            </footer>
        </Transition>
    }
}

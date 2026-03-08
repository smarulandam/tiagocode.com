use leptos::either::Either;
use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::controllers::layout_controller;
use crate::adapters::driver::leptos_webui::views::components::common::Navbar;

#[component]
pub fn BasicLayout(children: Children) -> impl IntoView {
    let layout = OnceResource::new(layout_controller());

    view! {
        <Transition fallback=move || { view! { <div class="bg-white"></div> }}>
            <header class="nav-header" id="header">
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
            <main class="site-main">
                <div class="site-frame">
                    {children()}
                </div>
            </main>
            <footer class="site-footer">
                <div class="site-footer__inner">
                    <p class="site-footer__text">"Made with love by Santiago Marulanda ❤️."</p>
                </div>
            </footer>
        </Transition>
    }
}

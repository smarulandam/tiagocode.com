use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::Logo;
use crate::adapters::driver::leptos_webui::views::components::common::Menu;
use crate::application::domain::layout::MenuTree;

#[component]
pub fn Navbar(main_menu: MenuTree, social_menu: MenuTree) -> impl IntoView {
    let (open_mobile_menu, set_open_mobile_menu) = signal(false);

    view! {
        <nav aria-label="Primary navigation">
            <div class="nav-shell flex-wrap md:flex-nowrap">
                <Logo />

                <div class="hidden items-center gap-3 md:flex">
                    <div>
                        <Menu
                            items=main_menu.items().clone()
                            container_class="nav-links-list"
                            anchor_class="nav-link"
                        />
                    </div>
                    <div class="border-l border-[rgba(181,200,202,0.68)] pl-3">
                        <Menu
                            items=social_menu.items().clone()
                            container_class="nav-social-list"
                            anchor_class="social-link"
                        />
                    </div>
                </div>

                <div class="flex items-center gap-2 md:hidden">
                    <Menu
                        items=social_menu.items().clone()
                        container_class="nav-social-list"
                        anchor_class="social-link"
                    />
                    <button
                        type="button"
                        class="menu-toggle"
                        aria-controls="mobile-menu"
                        aria-expanded=move || open_mobile_menu.get().to_string()
                        on:click=move |_| set_open_mobile_menu.update(|v| *v = !*v)
                    >
                        <span class="sr-only">
                            {move || if open_mobile_menu.get() { "Close main menu" } else { "Open main menu" }}
                        </span>
                        {move || {
                            if open_mobile_menu.get() {
                                view! {
                                    <svg
                                        class="h-5 w-5"
                                        aria-hidden="true"
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke="currentColor"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M6 6l12 12M18 6L6 18"
                                        />
                                    </svg>
                                }.into_any()
                            } else {
                                view! {
                                    <svg
                                        class="h-5 w-5"
                                        aria-hidden="true"
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke="currentColor"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M4 7h16M4 12h16M4 17h16"
                                        />
                                    </svg>
                                }.into_any()
                            }
                        }}
                    </button>
                </div>
                <div
                    id="mobile-menu"
                    class="mobile-nav-panel md:hidden"
                    class:hidden=move || !open_mobile_menu.get()
                >
                    <Menu
                        items=main_menu.items().clone()
                        anchor_class="mobile-nav-link"
                        container_class="mobile-nav-list"
                    />
                </div>
            </div>
        </nav>
    }
}

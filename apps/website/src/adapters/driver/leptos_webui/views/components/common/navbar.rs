use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::Logo;
use crate::adapters::driver::leptos_webui::views::components::common::Menu;
use crate::application::domain::layout::MenuTree;

#[component]
pub fn Navbar(main_menu: MenuTree, social_menu: MenuTree) -> impl IntoView {
    let (open_mobile_menu, set_open_mobile_menu) = signal(false);

    view! {
        <nav>
            <div class="flex flex-wrap items-center justify-between max-w-[1320px] mx-auto py-4 px-5 xl:px-0">
                <Logo />
                <div class="flex items-center">
                    <div class="md:order-2">
                        <Menu
                            items=social_menu.items().clone()
                            item_class="block py-0 px-3 -mr-3"
                            container_class="flex items-center font-medium"
                        />
                    </div>
                    <div class="md:order-1 hidden md:block">
                        <Menu
                            items=main_menu.items().clone()
                            item_class="block py-0 px-3 uppercase"
                            container_class="flex items-center font-medium mr-4"
                        />
                    </div>
                </div>
                <div class="block md:hidden">
                    <button
                        type="button"
                        class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg bg-white hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200"
                        aria-controls="mobile-menu"
                        aria-expanded=move || open_mobile_menu.get().to_string()
                        on:click=move |_| set_open_mobile_menu.update(|v| *v = !*v)
                    >
                        <span class="sr-only">Open main menu</span>
                        <svg
                            class="w-5 h-5"
                            aria-hidden="true"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 17 14"
                        >
                            <path
                                stroke="currentColor"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M1 1h15M1 7h15M1 13h15"
                            />
                        </svg>
                    </button>
                </div>
                <div
                    id="mobile-menu"
                    class="w-full md:hidden md:w-auto"
                    class:hidden=move || !open_mobile_menu.get()
                >
                    <Menu
                        items=main_menu.items().clone()
                        anchor_class="block"
                        item_class="block py-2 px-3 text-gray-900 rounded-sm hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:p-0 uppercase"
                        container_class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white"
                    />
                </div>
            </div>
        </nav>
    }
}

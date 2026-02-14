use leptos::prelude::*;

#[component]
pub fn UnexpectedError() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center p-6 text-center text-gray-800 rounded-lg max-w-md mx-auto">
            <svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 256 256" xml:space="preserve">
                <g transform="translate(1.4 1.4) scale(2.81 2.81)" fill="none">
                    <path d="M 45 90 C 20.187 90 0 69.813 0 45 C 0 20.187 20.187 0 45 0 c 24.813 0 45 20.187 45 45 C 90 69.813 69.813 90 45 90 z M 45 4 C 22.393 4 4 22.393 4 45 s 18.393 41 41 41 s 41 -18.393 41 -41 S 67.607 4 45 4 z" fill="gray"/>
                    <circle cx="30.344" cy="33.274" r="5.864" fill="gray"/>
                    <circle cx="59.664" cy="33.274" r="5.864" fill="gray"/>
                    <path d="M 72.181 65.49 c -0.445 0 -0.893 -0.147 -1.265 -0.451 c -7.296 -5.961 -16.5 -9.244 -25.916 -9.244 c -9.417 0 -18.62 3.283 -25.916 9.244 c -0.854 0.7 -2.115 0.572 -2.814 -0.283 c -0.699 -0.855 -0.572 -2.115 0.283 -2.814 C 24.561 55.398 34.664 51.795 45 51.795 c 10.336 0 20.438 3.604 28.447 10.146 c 0.855 0.699 0.982 1.959 0.283 2.814 C 73.335 65.239 72.76 65.49 72.181 65.49 z" fill="gray"/>
                </g>
            </svg>
            <h2 class="text-2xl font-bold tracking-wide mt-4">"Oops! Something went wrong"</h2>
            <p class="mt-2 text-gray-600">"An unexpected error occurred. Please try again later or go back to the homepage."</p>
            <div class="flex gap-3 mt-4">
                <button class="px-5 py-2 bg-gray-300 text-gray-800 font-medium rounded-full shadow-md hover:bg-gray-400 transition duration-200" onclick="window.location.href='/'">
                    Go Home
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn NotFoundError() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center p-6 text-center text-gray-800 rounded-lg max-w-md mx-auto">
            <svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 256 256" xml:space="preserve">
                <g transform="translate(1.4 1.4) scale(2.81 2.81)" fill="none">
                    <path d="M 45 90 C 20.187 90 0 69.813 0 45 C 0 20.187 20.187 0 45 0 c 24.813 0 45 20.187 45 45 C 90 69.813 69.813 90 45 90 z M 45 4 C 22.393 4 4 22.393 4 45 s 18.393 41 41 41 s 41 -18.393 41 -41 S 67.607 4 45 4 z" fill="gray"/>
                    <circle cx="30.344" cy="33.274" r="5.864" fill="gray"/>
                    <circle cx="59.664" cy="33.274" r="5.864" fill="gray"/>
                    <path d="M 72.181 65.49 c -0.445 0 -0.893 -0.147 -1.265 -0.451 c -7.296 -5.961 -16.5 -9.244 -25.916 -9.244 c -9.417 0 -18.62 3.283 -25.916 9.244 c -0.854 0.7 -2.115 0.572 -2.814 -0.283 c -0.699 -0.855 -0.572 -2.115 0.283 -2.814 C 24.561 55.398 34.664 51.795 45 51.795 c 10.336 0 20.438 3.604 28.447 10.146 c 0.855 0.699 0.982 1.959 0.283 2.814 C 73.335 65.239 72.76 65.49 72.181 65.49 z" fill="gray"/>
                </g>
            </svg>
            <h2 class="text-2xl font-bold tracking-wide mt-4">"Oops! Page Not Found"</h2>
            <p class="mt-2 text-gray-600">"The page you're looking for doesn't exist or has been moved."</p>
            <div class="flex gap-3 mt-4">
                <button class="mt-4 px-5 py-2 bg-gray-800 text-white font-medium rounded-full shadow-md hover:bg-gray-700 transition duration-200" onclick="window.location.href='/'">
                    Go Home
                </button>
            </div>
        </div>
    }
}

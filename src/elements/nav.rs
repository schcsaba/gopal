use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let location = use_location();
    let is_home = move || location.pathname.get() == "/";
    let is_menu = move || location.pathname.get() == "/menu";
    let is_reservation = move || location.pathname.get() == "/reservation";
    let is_contact = move || location.pathname.get() == "/contact";

    let set_if_show_modal =
        use_context::<WriteSignal<bool>>().expect("to have found the setter provided");

    let if_show_announcement_button = std::env::var("IF_SHOW_ANNOUNCEMENT_BUTTON")
        .map(|v| v.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);

    view! {
        <nav
            class=("text-white", move || is_home())
            class=("text-black", move || !is_home())
            class="relative z-20 top-0 right-0 w-full inline-block px-10 pt-5 text-2xl bg-transparent bg-opacity-50 rounded-xl"
        >
            <div class="flex justify-between items-center">
                <div>
                    {
                        move || if is_home() {
                            view! {
                                <img alt="Gopal Logo" class="h-20 lg:h-32" src="/assets/img/gopal-logo-white.svg" />
                            }.into_any()
                        } else {
                            view! {
                                <a href="/">
                                    <img alt="Gopal Logo" class="h-20 lg:h-32" src="/assets/img/gopal-logo-black.svg" />
                                </a>
                            }.into_any()
                        }
                    }
                </div>
                <div class="flex">
                    <Show when=move || { if_show_announcement_button }>
                        <button on:click=move |_| set_if_show_modal(true) class="animate-bounce text-violet-700 bg-gradient-to-r from-lime-200 via-lime-400 to-lime-500 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-lime-300 dark:focus:ring-lime-800 shadow-lg shadow-lime-500/50 dark:shadow-lg dark:shadow-lime-800/80 font-medium rounded-lg text-lg px-5 py-2.5 text-center me-2 mb-2">
                            "Annonce"
                        </button>
                    </Show>
                    <div class="hidden lg:block">
                        <ul class="flex">
                            <li class="px-4 py-4">
                                <a class="tracking-widest uppercase" class=("text-gray-300", move || is_menu()) href="/menu">Menu</a>
                            </li>
                            <li class="px-4 py-4">
                                <a class="tracking-widest uppercase" class=("text-gray-300", move || is_reservation()) href="/reservation">Réservation</a>
                            </li>
                            <li class="px-4 py-4">
                                <a class="tracking-widest uppercase" class=("text-gray-300", move || is_contact()) href="/contact">Contact</a>
                            </li>
                        </ul>
                    </div>
                    <div class="block lg:hidden">
                        <button
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class:transition=move || is_open()
                            class=("transform-180", move || is_open())
                            class="block lg:hidden px-2 text-gray-500 hover:text-white focus:outline-none focus:text-white"
                            type="button"
                            id="mobile-menu-button"
                            aria-label="Mobile menu button"
                        >
                            <svg
                            class="h-6 w-6 fill-current"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                            >
                            {move || if is_open() {
                                view! {
                                    <path
                                        clip-rule="evenodd"
                                        d="M18.278 16.864a1 1 0 0 1-1.414 1.414l-4.829-4.828-4.828 4.828a1 1 0 0 1-1.414-1.414l4.828-4.829-4.828-4.828a1 1 0 0 1 1.414-1.414l4.829 4.828 4.828-4.828a1 1 0 1 1 1.414 1.414l-4.828 4.829 4.828 4.828z"
                                        fill-rule="evenodd"
                                        fill=move || if is_home() {
                                            "white"
                                        } else {
                                            "black"
                                        }
                                    />
                                }
                            } else {
                                view! {
                                    <path
                                        d="M4 5h16a1 1 0 0 1 0 2H4a1 1 0 1 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2z"
                                        fill-rule="evenodd"
                                        fill=move || if is_home() {
                                            "white"
                                        } else {
                                            "black"
                                        }
                                    />
                                }
                            }}
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
            <div
                class=("max-h-0", move || !is_open())
                class=("max-h-screen", move || is_open())
                class="w-full z-50 lg:hidden flex justify-end w-auto overflow-hidden transition-all duration-400 ease-in-out"
            >
                <ul
                    class=("bg-lime-200/65", move || is_home())
                    class=("rounded-3xl", move || is_home())
                    class="pt-6 pt-0 list-reset flex flex-col items-end"
                >
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class=("text-white", move || is_home())
                            class=("hover:text-gray-200", move || !is_home())
                            class="inline-block py-2 px-4 no-underline uppercase"
                            href="/"
                        >
                            Accueil
                        </a>
                    </li>
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class=("text-gray-200", move || is_menu())
                            class=("text-gray-600", move || !is_menu())
                            class="inline-block no-underline hover:text-gray-200 py-2 px-4 uppercase"
                            href="/menu"
                        >
                            Menu
                        </a>
                    </li>
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class=("text-gray-200", move || is_reservation())
                            class=("text-gray-600", move || !is_reservation())
                            class="inline-block no-underline hover:text-gray-200 py-2 px-4 uppercase"
                            href="/reservation"
                        >
                            Réservation
                        </a>
                    </li>
                        <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class=("text-gray-200", move || is_contact())
                            class=("text-gray-600", move || !is_contact())
                            class="inline-block no-underline hover:text-gray-200 py-2 px-4 uppercase"
                            href="/contact"
                        >
                            Contact
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

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
                                <img alt="Gopal Logo" class="h-20 lg:h-32" src="assets/img/gopal-logo-white.svg" />
                            }.into_any()
                        } else {
                            view! {
                                <a href="/">
                                    <img alt="Gopal Logo" class="h-20 lg:h-32" src="assets/img/gopal-logo-black.svg" />
                                </a>
                            }.into_any()
                        }
                    }
                </div>
                <div class="hidden lg:block">
                    <ul class="flex">
                        <li class="px-4 py-4">
                            <a class="tracking-widest" href="menu">MENU</a>
                        </li>
                        <li class="px-4 py-4">
                            <a class="tracking-widest" href="reservation">RESERVATION</a>
                        </li>
                        <li class="px-4 py-4">
                            <a class="tracking-widest" href="contact">CONTACT</a>
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
            <div
                class=("max-h-0", move || !is_open())
                class=("max-h-screen", move || is_open())
                class="w-full h-full z-50 lg:hidden lg:flex lg:items-center lg:w-auto text-right overflow-hidden transition-all duration-400 ease-in-out"
            >
                <ul class="pt-6 lg:pt-0 list-reset lg:flex justify-start flex-1 items-center">
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class=("text-white", move || is_home())
                            class=("hover:text-gray-200", move || !is_home())
                            class=("hover:text-underline", move || !is_home())
                            class="inline-block py-2 px-4 no-underline"
                            href="/"
                        >
                            HOME
                        </a>
                    </li>
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class=("text-gray-200", move || is_menu())
                            class=("text-gray-600", move || !is_menu())
                            class="inline-block no-underline hover:text-gray-200 hover:text-underline py-2 px-4"
                            href="menu"
                        >
                            MENU
                        </a>
                    </li>
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class=("text-gray-200", move || is_reservation())
                            class=("text-gray-600", move || !is_reservation())
                            class="inline-block no-underline hover:text-gray-200 hover:text-underline py-2 px-4"
                            href="reservation"
                        >
                            RESERVATION
                        </a>
                    </li>
                        <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = !*o);
                            }
                            class=("text-gray-200", move || is_contact())
                            class=("text-gray-600", move || !is_contact())
                            class="inline-block no-underline hover:text-gray-200 hover:text-underline py-2 px-4"
                            href="contact"
                        >
                            CONTACT
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

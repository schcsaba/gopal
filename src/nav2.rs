use leptos::*;

#[component]
pub fn Nav2() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    view! {
        <nav class="relative z-20 top-0 right-0 w-full inline-block px-10 pt-5 text-2xl text-black bg-transparent bg-opacity-50 rounded-xl">
            <div class="flex justify-between items-center">
                <div>
                    <a href="/">
                        <img alt="Picnic Bistro Logo" class="h-20 lg:h-32" src="assets/img/picnic-logo-black.svg" />
                    </a>
                </div>
                <div class="hidden lg:block">
                    <ul class="flex">
                        <li class="px-4 py-4">
                            <a class="tracking-widest font-thin" href="menu">MENU</a>
                        </li>
                        <li class="px-4 py-4">
                            <a class="tracking-widest" href="rezervari.html">REZERVATION</a>
                        </li>
                        <li class="px-4 py-4">
                            <a class="tracking-widest" href="contact.html">CONTACT</a>
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
                                fill="black"
                            />
                        }
                    } else {
                        view! {
                            <path
                                d="M4 5h16a1 1 0 0 1 0 2H4a1 1 0 1 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2z"
                                fill-rule="evenodd"
                                fill="black"
                            />
                        }
                    }}
                    </svg>
                  </button>
                </div>
            </div>
            <div
                class:block=move || is_open()
                class:hidden=move || !is_open()
                class="w-full h-full z-50 lg:hidden lg:flex lg:items-center lg:w-auto text-right"
            >
                <ul class="pt-6 lg:pt-0 list-reset lg:flex justify-start flex-1 items-center">
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = false);
                            }
                            class="inline-block py-2 px-4 text-gray-600 no-underline hover:text-gray-200 hover:text-underline"
                            href="/"
                        >
                            HOME
                        </a>
                    </li>
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = true);
                            }
                            class="inline-block text-gray-200 no-underline hover:text-gray-200 hover:text-underline py-2 px-4"
                            href="menu"
                        >
                            MENU
                        </a>
                    </li>
                    <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = true);
                            }
                            class="inline-block text-gray-600 no-underline hover:text-gray-200 hover:text-underline py-2 px-4"
                            href="rezervari.html"
                        >
                            REZERVATION
                        </a>
                    </li>
                        <li class="mr-3">
                        <a
                            on:click=move |_| {
                                set_is_open.update(|o| *o = false);
                            }
                            class="inline-block text-gray-600 no-underline hover:text-gray-200 hover:text-underline py-2 px-4"
                            href="contact.html"
                        >
                            CONTACT
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

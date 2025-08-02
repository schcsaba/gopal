use leptos::ev::{MouseEvent, TouchEvent};
use leptos::prelude::*;

#[component]
pub fn Modal<F, IV>(
    set_if_show_modal: WriteSignal<bool>,
    title: String,
    content: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let on_close = move |_| {
        set_if_show_modal(false);
    };

    let on_close_touch = move |_| {
        set_if_show_modal(false);
    };

    // Handle both mouse and touch events for closing
    let on_backdrop_interact = move |ev: MouseEvent| {
        ev.stop_propagation();
        set_if_show_modal(false);
    };

    let on_backdrop_touch = move |ev: TouchEvent| {
        ev.stop_propagation();
        // Prevent default to avoid any scrolling issues
        ev.prevent_default();
        set_if_show_modal(false);
    };

    // Prevent event bubbling for the modal content
    let stop_propagation = move |ev: MouseEvent| {
        ev.stop_propagation();
    };

    let stop_touch_propagation = move |ev: TouchEvent| {
        ev.stop_propagation();
    };

    view! {
        <div
            on:click=on_backdrop_interact
            on:touchend=on_backdrop_touch
            class="fixed inset-0 z-30"
            aria-labelledby="modal-title"
            role="dialog"
            aria-modal="true"
            style="touch-action: none;"
        >
            <div
                class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
                aria-hidden="true"
            />

            <div class="fixed inset-0 z-30 w-screen overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div
                        on:click=stop_propagation
                        on:touchend=stop_touch_propagation
                        class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full lg:max-w-xl md:max-w-xl sm:max-w-lg"
                        style="touch-action: manipulation;"
                    >
                        <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                            <div class="sm:flex sm:items-start">
                                <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                                    <h3 class="text-xl font-bold leading-6 text-red-500" id="modal-title">{ title }</h3>
                                    <div class="mt-2">
                                        <div class="text-sm text-gray-500">{ content() }</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                            <button
                                on:click=on_close
                                on:touchend=on_close_touch
                                type="button"
                                class="mt-3 inline-flex w-full cursor-pointer justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto"
                                style="touch-action: manipulation;"
                            >
                                Fermer
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

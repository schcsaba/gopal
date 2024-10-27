use leptos::ev::MouseEvent;
use leptos::*;

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

    let stop_propagation = move |ev: MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        <div on:click=on_close class="relative z-30 hover:cursor-pointer" aria-labelledby="modal-title" role="dialog" aria-modal="true">

            <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" aria-hidden="true"></div>

            <div class="fixed inset-0 z-30 w-screen overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div on:click=stop_propagation class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full lg:max-w-xl md:max-w-xl sm:max-w-lg">
                        <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                            <div class="sm:flex sm:items-start">
                                <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                                    <h3 class="text-xl font-bold leading-6 text-gray-900" id="modal-title">{ title }</h3>
                                    <div class="mt-2">
                                        <div class="text-sm text-gray-500">{ content() }</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                            <button on:click=on_close type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto">Fermer</button>
                        </div>
                    </div>
                </div>
            </div>
      </div>
    }
}

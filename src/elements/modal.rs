use leptos::prelude::*;

/// Rendered hidden; `public/site.js` opens it from any `data-modal-open`
/// button and closes it on the backdrop or the close button.
#[component]
pub fn Modal<F, IV>(title: String, content: F) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div
            data-modal
            class="hidden fixed inset-0 z-30"
            aria-labelledby="modal-title"
            role="dialog"
            aria-modal="true"
            style="touch-action: none;"
        >
            <div
                class="fixed inset-0 bg-encre/70 transition-opacity"
                aria-hidden="true"
            />

            <div class="fixed inset-0 z-30 w-screen overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div
                        data-modal-panel
                        class="relative transform overflow-hidden border-[1.5px] border-encre bg-papier text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg md:max-w-xl"
                        style="touch-action: manipulation;"
                    >
                        <div class="px-5 pt-6 pb-4 sm:p-8 sm:pb-4">
                            <div class="sm:flex sm:items-start">
                                <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                                    <h3 class="font-serif text-3xl leading-tight text-piment" id="modal-title">{ title }</h3>
                                    <div class="mt-4">
                                        <div class="text-lg text-mousse">{ content() }</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="border-t border-ligne px-5 py-4 sm:flex sm:flex-row-reverse sm:px-8">
                            <button
                                data-modal-close
                                type="button"
                                class="btn btn-encre w-full cursor-pointer sm:w-auto"
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

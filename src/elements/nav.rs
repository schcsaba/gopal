use leptos::prelude::*;
use leptos_router::hooks::use_location;

/// Whether to show the "Annonce" button and modal, decided at build time.
pub fn announcement_enabled() -> bool {
    std::env::var("IF_SHOW_ANNOUNCEMENT_BUTTON")
        .map(|v| v.parse::<bool>().unwrap_or(false))
        .unwrap_or(false)
}

/// `aria-current` for a link to `href`, so the current page is announced
/// and underlined.
fn current(path: &str, href: &str) -> Option<&'static str> {
    (path == href).then_some("page")
}

// The mobile menu toggle is wired up by `public/site.js` through the
// `data-menu-*` attributes.
#[component]
pub fn Nav() -> impl IntoView {
    let path = use_location().pathname.get_untracked();
    let is_home = path == "/";
    let desktop_link = |href: &'static str, label: &'static str| {
        let here = current(&path, href);
        view! {
            <li>
                <a
                    href=href
                    aria-current=here
                    class="py-3 hover:text-piment"
                    class=("nav-courant", here.is_some())
                >
                    {label}
                </a>
            </li>
        }
    };
    let mobile_link = |href: &'static str, label: &'static str| {
        let here = current(&path, href);
        view! {
            <li class="border-t border-ligne">
                <a
                    href=href
                    aria-current=here
                    class="flex items-center justify-between py-4 font-serif text-4xl hover:text-piment"
                    class=("italic", here.is_some())
                    class=("text-piment", here.is_some())
                >
                    {label}
                    <span aria-hidden="true" class="font-sans text-2xl">"→"</span>
                </a>
            </li>
        }
    };

    view! {
        <nav aria-label="Navigation principale" class="wrap">
            <div class="flex items-center justify-between gap-6 py-3 lg:min-h-28">
                {if is_home {
                    view! {
                        <img alt="Gopal Logo" class="h-16 w-auto lg:h-[5.5rem]" width="104" height="87" src="/assets/img/gopal-logo-encre.svg" />
                    }.into_any()
                } else {
                    view! {
                        <a href="/" aria-label="Accueil" class="shrink-0">
                            <img alt="Gopal Logo" class="h-16 w-auto lg:h-[5.5rem]" width="104" height="87" src="/assets/img/gopal-logo-encre.svg" />
                        </a>
                    }.into_any()
                }}
                <div class="flex items-center gap-4 lg:gap-10">
                    <Show when=announcement_enabled>
                        <button
                            data-modal-open
                            type="button"
                            class="btn min-h-12 bg-curcuma text-encre hover:bg-curcuma-clair lg:min-h-13"
                        >
                            "Annonce"
                        </button>
                    </Show>
                    <ul class="hidden items-center gap-10 text-[1.1875rem] font-semibold uppercase tracking-[0.12em] lg:flex">
                        {desktop_link("/menu", "Menu")}
                        {desktop_link("/contact", "Contact")}
                        <li>
                            <a
                                href="/reservation"
                                aria-current=current(&path, "/reservation")
                                class="btn btn-piment min-h-13 tracking-[0.12em]"
                            >
                                "Réservation"
                            </a>
                        </li>
                    </ul>
                    <button
                        data-menu-toggle
                        aria-expanded="false"
                        aria-controls="mobile-menu"
                        class="flex size-12 items-center justify-center rounded-full border-[1.5px] border-encre text-encre hover:bg-encre hover:text-papier lg:hidden"
                        type="button"
                        id="mobile-menu-button"
                        aria-label="Mobile menu button"
                    >
                        <svg class="size-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
                            <path data-menu-icon="close" class="hidden" d="M6 6l12 12M18 6L6 18" />
                            <path data-menu-icon="open" d="M3 6h18M3 12h18M3 18h18" />
                        </svg>
                    </button>
                </div>
            </div>
            <div
                id="mobile-menu"
                data-menu
                class="max-h-0 overflow-hidden transition-[max-height] duration-500 ease-in-out lg:hidden"
            >
                <ul class="mb-6 border-b border-ligne">
                    {mobile_link("/", "Accueil")}
                    {mobile_link("/menu", "Menu")}
                    {mobile_link("/reservation", "Réservation")}
                    {mobile_link("/contact", "Contact")}
                </ul>
            </div>
        </nav>
    }
}

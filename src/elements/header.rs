use crate::data::contact::get_contact_info;
use crate::data::noel::{ORDER_DEADLINE, PHONE, PHONE_DISPLAY, SHOP_URL};
use crate::elements::nav::Nav;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

pub const INSTAGRAM: &str = "https://www.instagram.com/le_gopal/";
pub const FACEBOOK: &str = "https://www.facebook.com/Legopaltours";
pub const MESSENGER: &str = "https://m.me/Legopaltours";

/// The thin address bar above the navigation, then the navigation itself.
#[component]
pub fn Header() -> impl IntoView {
    let contact = get_contact_info();
    let address = contact.address.trim_end_matches(", France").to_string();
    let phone = contact.format_phone_for_display();
    let phone_href = format!("tel:{}", contact.phone);

    view! {
        <NoelBanner />
        <div class="hidden bg-encre text-[0.9375rem] tracking-wide text-lin md:block">
            <div class="wrap flex min-h-11 flex-wrap items-center justify-between gap-x-6">
                <p>
                    {address} " · "
                    <a href=phone_href class="hover:text-curcuma-clair">{phone}</a>
                </p>
                <p>
                    <a href=INSTAGRAM target="_blank" rel="noreferrer noopener" class="hover:text-curcuma-clair">"instagram"</a>
                    " / "
                    <a href=FACEBOOK target="_blank" rel="noreferrer noopener" class="hover:text-curcuma-clair">"facebook"</a>
                    " / "
                    <a href=MESSENGER target="_blank" rel="noreferrer noopener" class="hover:text-curcuma-clair">"messenger"</a>
                </p>
            </div>
        </div>
        <header class="border-b border-ligne bg-papier">
            <Nav />
        </header>
    }
}

/// The Christmas menu and bûches, announced on every page but their own. Seasonal: remove
/// it with `pages/noel.rs` once the season is over.
#[component]
fn NoelBanner() -> impl IntoView {
    let on_noel = use_location().pathname.get_untracked() == "/noel";
    (!on_noel).then(|| view! {
        <a href="/noel" class="group block bg-piment text-creme hover:bg-piment-fonce">
            <span class="wrap flex min-h-12 flex-wrap items-center justify-center gap-x-3 py-2 text-center text-[1.0625rem] lg:text-lg">
                <span aria-hidden="true" class="text-curcuma-clair">"✦"</span>
                <span>
                    <strong class="font-serif text-xl font-medium italic lg:text-[1.375rem]">"Menu & bûches de Noël à emporter"</strong>
                    <span class="hidden sm:inline">{format!(" · commandes jusqu’au {ORDER_DEADLINE}")}</span>
                </span>
                <span class="font-semibold underline decoration-curcuma-clair decoration-2 underline-offset-4 group-hover:text-curcuma-clair">
                    "Découvrir" <span aria-hidden="true">"→"</span>
                </span>
            </span>
        </a>
    })
}

/// Call and book buttons kept at the bottom of the screen on phones. On the
/// reservation page, where booking is already in front of the visitor, the
/// second button opens Messenger instead; on the Christmas menu page, the
/// buttons call the number for Christmas orders and open the online shop.
#[component]
pub fn MobileBar() -> impl IntoView {
    let contact = get_contact_info();
    let path = use_location().pathname.get_untracked();
    let on_noel = path == "/noel";
    let (phone, phone_href) = if on_noel {
        (PHONE_DISPLAY.to_string(), format!("tel:{PHONE}"))
    } else {
        (contact.format_phone_for_display(), format!("tel:{}", contact.phone))
    };

    view! {
        <div class="fixed inset-x-0 bottom-0 z-20 flex gap-2.5 bg-encre px-4 pt-3 pb-[max(1rem,env(safe-area-inset-bottom))] lg:hidden">
            <a href=phone_href class="btn btn-contour min-h-13 flex-1 border-lin px-3 text-[1.0625rem] tracking-normal text-papier normal-case hover:border-curcuma hover:bg-transparent hover:text-curcuma-clair">
                {phone}
            </a>
            {if on_noel {
                view! {
                    <a href=SHOP_URL class="btn btn-piment min-h-13 flex-1 px-3 text-[1.0625rem] tracking-wide">
                        "Commander"
                    </a>
                }.into_any()
            } else if path == "/reservation" {
                view! {
                    <a href=MESSENGER target="_blank" rel="noreferrer noopener" class="btn btn-piment min-h-13 flex-1 px-3 text-[1.0625rem] tracking-normal normal-case">
                        "Messenger"
                    </a>
                }.into_any()
            } else {
                view! {
                    <a href="/reservation" class="btn btn-piment min-h-13 flex-1 px-3 text-[1.0625rem] tracking-wide">
                        "Réservation"
                    </a>
                }.into_any()
            }}
        </div>
    }
}

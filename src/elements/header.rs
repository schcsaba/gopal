use crate::data::contact::get_contact_info;
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

/// Call and book buttons kept at the bottom of the screen on phones. On the
/// reservation page, where booking is already in front of the visitor, the
/// second button opens Messenger instead.
#[component]
pub fn MobileBar() -> impl IntoView {
    let contact = get_contact_info();
    let phone = contact.format_phone_for_display();
    let phone_href = format!("tel:{}", contact.phone);
    let on_reservation = use_location().pathname.get_untracked() == "/reservation";

    view! {
        <div class="fixed inset-x-0 bottom-0 z-20 flex gap-2.5 bg-encre px-4 pt-3 pb-[max(1rem,env(safe-area-inset-bottom))] lg:hidden">
            <a href=phone_href class="btn btn-contour min-h-13 flex-1 border-lin px-3 text-[1.0625rem] tracking-normal text-papier normal-case hover:border-curcuma hover:bg-transparent hover:text-curcuma-clair">
                {phone}
            </a>
            {if on_reservation {
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

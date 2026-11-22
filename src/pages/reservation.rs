use crate::data::contact::{get_contact_info, OpeningHours};
use crate::elements::header::MESSENGER;
use leptos::prelude::*;

#[component]
pub fn Reservation() -> impl IntoView {
    let contact = get_contact_info();
    let phone_formatted = contact.format_phone_for_display();
    let phone_href = format!("tel:{}", contact.phone);
    let email_href = format!("mailto:{}", contact.email);
    let line = "text-xl lg:text-[1.375rem]";

    view! {
        <main id="contenu">
            <section class="bg-papier">
                <div class="wrap py-10 lg:py-18">
                    <h1 class="font-serif text-5xl leading-none font-normal tracking-[-0.02em] sm:text-6xl lg:text-8xl">
                        "Réservez une " <em class="text-piment">"table"</em>
                    </h1>
                </div>
                <div class="wrap grid items-start gap-14 pb-20 lg:grid-cols-[minmax(0,1.35fr)_minmax(0,1fr)] lg:gap-18 lg:pb-24">
                    <div class="border-[1.5px] border-encre bg-white">
                        <iframe src="https://widget.thefork.com/60d93c62-f1bb-412e-9206-93ab3a1900cb" allow="payment *" title="TheFork" class="block w-full" style="min-height: 800px; border: none; overflow: scroll;"></iframe>
                    </div>
                    <div class="flex flex-col gap-12">
                        <div>
                            <h2 class="eyebrow pb-3 text-[0.9375rem] lg:text-base">"Heures d'ouverture : "</h2>
                            <HoursList hours=contact.hours.clone() />
                        </div>
                        <div class="flex flex-col gap-3">
                            <h2 class="pb-2 font-serif text-3xl leading-tight font-medium lg:text-4xl">"Sur place ou à emporter ou livraison à domicile"</h2>
                            <p class=line>"Livraison à domicile : " <a class="lien" href="https://www.frerestoque.fr/shop/angers-centre-ville/gopal" target="_blank" rel="noreferrer">"Cliquez ici"</a></p>
                            <p class=line>"Tél : " <a class="lien" href=phone_href>{phone_formatted}</a></p>
                            <p class=line>"Email : " <a class="lien" href=email_href>{contact.email.clone()}</a></p>
                            <p class=line>"Messenger : " <a class="lien" href=MESSENGER target="_blank" rel="noreferrer">"LeGopaltours"</a></p>
                            <p class="pt-3 text-lg text-lichen">"Méthodes de paiement : Espèces · Visa · Mastercard · Ticket restaurant"</p>
                        </div>
                        <div class="flex items-center gap-6 border-y border-ligne py-6">
                            <img alt="Table de pique-nique" class="h-20 w-auto shrink-0 opacity-80" src="/assets/img/picnic.svg" />
                            <p class="font-serif text-2xl leading-snug text-piment italic lg:text-[1.75rem]">"Tous nos plats sont faits maison."</p>
                        </div>
                    </div>
                </div>
            </section>

            <section class="bg-sable">
                <div class="wrap flex flex-wrap-reverse items-center gap-12 py-20 lg:gap-18 lg:py-24">
                    <div class="frame frame-tl min-w-0 flex-[1_1_32rem]">
                        <img class="block h-auto w-full" loading="lazy" width=827 height=591 alt="Carte cadeau" src="/assets/img/giftcard.webp" />
                    </div>
                    <div class="flex min-w-0 flex-[1_1_24rem] flex-col gap-5">
                        <h2 class="font-serif text-5xl leading-[1.02] font-medium lg:text-6xl">"Nos cartes cadeaux !!! 🎁😄"</h2>
                        <p class="text-xl lg:text-2xl">"En vente au Gopal !"</p>
                        <p class="font-serif text-2xl text-piment-fonce italic lg:text-3xl">"Faites plaisir à coup sûr !"</p>
                    </div>
                </div>
            </section>
        </main>
    }
}

/// Opening hours as a ruled list, the day with an evening service in the
/// accent colour.
#[component]
pub fn HoursList(hours: Vec<OpeningHours>) -> impl IntoView {
    view! {
        <ul class="text-lg lg:text-xl">
            {hours.into_iter().map(|day| {
                let evening = day.slots.len() > 1;
                view! {
                    <li class="flex items-start justify-between gap-4 border-b border-ligne py-2.5 last:border-b-0">
                        <span class="font-serif text-xl lg:text-[1.375rem]" class=("text-piment", evening)>{day.day.to_french()}</span>
                        <span class="text-right">
                            {match day.slots.as_slice() {
                                [one] => one.format().into_any(),
                                [first, second] => view! { {first.format()} <br class="sm:hidden" /> " et " {second.format()} }.into_any(),
                                _ => "Fermé".into_any(),
                            }}
                        </span>
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}

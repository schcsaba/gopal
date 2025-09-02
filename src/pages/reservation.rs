use leptos::prelude::*;
use crate::data::contact::get_contact_info;

#[component]
pub fn Reservation() -> impl IntoView {
    let contact = get_contact_info();
    let phone_formatted = contact.format_phone_for_display();
    let phone_href = format!("tel:{}", contact.phone);
    let email_href = format!("mailto:{}", contact.email);
    let hours = contact.hours.clone();
    view! {
        <main class="container mx-auto">
            <section class="container mx-auto my-20">
                <div class="text-center">
                    <h2 class="font-sans font-bold text-4xl pt-6 pb-6 uppercase tracking-wide">Réservez une table</h2>
                    <p class="text-red-500 font-bold">"Chers amis,"</p>
                    <p class="text-red-500 font-bold">"Le Gopal est fermé temporairement."</p>
                    <p class="text-red-500 font-bold">"Nous avons hâte de vous retrouver bientôt et vous remercions de votre fidélité et de votre patience."</p>
                    <iframe class="pb-6" src="https://widget.thefork.com/60d93c62-f1bb-412e-9206-93ab3a1900cb" allow="payment *" title="TheFork" style="width: 100%; min-height: 800px; border: none; overflow: scroll;"></iframe>
                    <h2 class="font-sans font-bold text-3xl pb-6 uppercase tracking-wide">Sur place ou à emporter ou livraison à domicile</h2>
                    <p class="text-2xl pb-4">Livraison à domicile : <span class="underline decoration-1 underline-offset-2"><a href="https://www.frerestoque.fr/shop/angers-centre-ville/gopal" target="_blank" rel="noreferrer">Cliquez ici</a></span></p>
                    <p class="text-2xl pb-4">Tél : <span class="underline decoration-1 underline-offset-2"><a href={phone_href}>{phone_formatted}</a></span></p>
                    <p class="text-2xl pb-4">Email : <span class="underline decoration-1 underline-offset-2"><a href={email_href}>{contact.email.clone()}</a></span></p>
                    <p class="text-2xl pb-6">Messenger : <span class="underline decoration-1 underline-offset-2"><a href="https://m.me/Legopaltours" target="_blank" rel="noreferrer">LeGopaltours</a></span></p>
                    <p class="text-xl pb-6">"Méthodes de paiement : Espèces · Visa · Mastercard · Ticket restaurant"</p>
                    <p class="text-2xl pb-6 font-extrabold">Tous nos plats sont faits maison.</p>
                </div>
                <div class="text-center my-10">
                    <h2 class="font-sans font-bold text-3xl pb-6 uppercase tracking-wide">"Heures d'ouverture : "</h2>
                    <div class="grid grid-cols-1 md:grid-cols-4 lg:grid-cols-5 gap-y-6 text-center">
                        <For
                            each=move || hours.clone()
                            key=|hours| format!("{:?}", hours.day)
                            children=move |hours| {
                                let day_name = hours.day.to_french();
                                let time_display = match hours.slots.len() {
                                    1 => hours.slots[0].format(),
                                    2 => format!("{} et {}", hours.slots[0].format(), hours.slots[1].format()),
                                    _ => "Fermé".to_string(),
                                };
                                view! {
                                    <div>
                                        <p class="font-sans font-bold text-2xl pb-2">
                                            {day_name}
                                        </p>
                                        <p class="text-xl">{time_display}</p>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
                <div class="flex items-center justify-center pb-8">
                    <img alt="Table de pique-nique" class="h-32" src="/assets/img/picnic.svg" />
                </div>
                <div class="text-center">
                    <h2 class="font-sans font-bold text-3xl pb-6 uppercase tracking-wide">"Nos cartes cadeaux !!! 🎁😄"</h2>
                    <p class="text-xl pb-6">En vente au Gopal !</p>
                    <p class="text-xl pb-6">Faites plaisir à coup sûr !</p>
                </div>
                <div class="flex items-center justify-center">
                    <div class="h-128">
                        <img width=827 height=591 alt="Carte cadeau" src="/assets/img/giftcard.webp" />
                    </div>
                </div>
            </section>
        </main>
    }
}

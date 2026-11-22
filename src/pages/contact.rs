use crate::data::contact::{format_opening_hours_text, get_contact_info};
use crate::elements::header::MESSENGER;
use crate::pages::home::MapEmbed;
use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    let contact = get_contact_info();
    let hours = contact.hours.clone();
    let address = contact.address.clone();
    let phone = contact.phone.clone();
    let email = contact.email.clone();
    let formatted_phone = contact.format_phone_for_display();

    let heading = "pb-4 font-serif text-3xl leading-tight font-medium lg:text-4xl";
    let line = "text-xl leading-relaxed";

    view! {
        <main id="contenu" class="bg-papier">
            <section class="wrap grid items-start gap-12 py-12 lg:grid-cols-2 lg:gap-18 lg:py-20">
                <div class="lg:sticky lg:top-8">
                    <div class="frame frame-br">
                        <div class="relative h-80 border-[1.5px] border-encre bg-kraft md:h-[28rem] lg:h-[40rem]">
                            <MapEmbed />
                        </div>
                    </div>
                </div>
                <div class="flex flex-col gap-12">
                    <div>
                        <h2 class=heading>"Heures d'ouverture :"</h2>
                        <ul class="border-t border-ligne">
                            {hours.into_iter().map(|hours| {
                                let evening = hours.slots.len() > 1;
                                view! {
                                    <li class="border-b border-ligne py-2.5 text-xl" class=("text-piment", evening)>
                                        {format_opening_hours_text(&hours)}
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    </div>
                    <div class="flex flex-col gap-2">
                        <h2 class=heading>"Coordonnées :"</h2>
                        <p class=line>{address}</p>
                        <p class=line>"Tél : " <a class="lien" href=format!("tel:{}", phone)>{formatted_phone.clone()}</a></p>
                        <p class=line>"Email : " <a class="lien" href=format!("mailto:{}", email)>{email.clone()}</a></p>
                        <p class=line>"Messenger : " <a class="lien" href=MESSENGER target="_blank" rel="noreferrer">"LeGopaltours"</a></p>
                        <p class="mt-4 border-l-[3px] border-curcuma pl-4 text-lg text-lichen">"Vous pouvez effectuer un zoom avant et arrière sur la carte à l'aide des boutons situés dans le coin inférieur droit ou avec deux doigts sur les appareils à écran tactile."</p>
                    </div>
                    <div class="flex flex-col gap-2">
                        <h2 class=heading>"Approche par les transports publics :"</h2>
                        <p class=line>"Le restaurant est à 180m de l'arrêt de tram Tranchée - Ligne A."</p>
                        <p class=line>"Autres lignes urbaines avec un arrêt à proximité : 10, 14, 17, 18, C2, 51, 52, 73"</p>
                        <p class=line>"Plus d'informations sur " <a class="lien" href="https://www.filbleu.fr/" target="_blank" rel="noreferrer">"Fil Bleu"</a> "."</p>
                        <p class=line>"Vous trouverez des informations sur le parking sur " <a class="lien" href="https://www.tours.fr/page-portail-ma-mairie/services-pratiques/deplacement/stationnement/" target="_blank" rel="noreferrer">"le site de la Ville de Tours"</a> "."</p>
                    </div>
                    <div class="ardoise on-ardoise px-8 py-9 sm:px-10">
                        <p class="text-xl leading-relaxed text-craie-douce">"Pour toute question ou réservation appelez nous au : "</p>
                        <p class="pt-2 font-serif text-4xl text-curcuma-clair lg:text-5xl">
                            <a class="hover:text-craie" href=format!("tel:{}", phone)>{formatted_phone.clone()}</a>
                        </p>
                    </div>
                </div>
            </section>
            <div class="flex min-h-64 items-center justify-center border-t border-ligne px-5 py-16">
                <p class="font-serif text-6xl italic lg:text-8xl">"À " <span class="text-piment">"bientôt"</span></p>
            </div>
        </main>
    }
}

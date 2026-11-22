use crate::data::contact::get_contact_info;
use crate::elements::header::{FACEBOOK, INSTAGRAM, MESSENGER};
use leptos::prelude::*;

const GOOGLE: &str = "https://www.google.com/search?hl=fr-FR&gl=fr&q=Gopal,+8+Av.+du+Mans,+37100+Tours";
const TRIPADVISOR: &str = "https://www.tripadvisor.fr/Restaurant_Review-g187130-d19250495-Reviews-Gopal-Tours_Indre_et_Loire_Centre_Val_de_Loire.html";

#[component]
pub fn Footer() -> impl IntoView {
    let contact = get_contact_info();
    let address_formatted = contact.address.replace(", ", "<br />");
    let phone_formatted = contact.format_phone_for_display();
    let phone_href = format!("tel:{}", contact.phone);
    let email_href = format!("mailto:{}", contact.email);
    let email = contact.email.clone();

    let heading = "pb-3 text-[0.9375rem] font-bold uppercase tracking-[0.2em] text-curcuma";
    let label = "pt-3 font-bold";
    let link = "hover:text-curcuma-clair";

    view! {
        <footer class="bg-encre text-lg leading-relaxed text-lin">
            <div class="wrap flex flex-wrap gap-12 pt-20 pb-36 *:min-w-0 *:flex-[1_1_15rem] lg:pt-22 lg:pb-14">
                <div>
                    <a href="/" aria-label="Accueil" class="inline-block">
                        <img alt="Gopal Logo" class="h-32 w-auto lg:h-40" width="190" height="158" src="/assets/img/gopal-logo-papier.svg" />
                    </a>
                </div>
                <nav aria-label="Plan du site">
                    <h2 class=heading>"Plan du site"</h2>
                    <ul class="flex flex-col gap-1">
                        <li><a class=link href="/">"Accueil"</a></li>
                        <li><a class=link href="/menu">"Menu"</a></li>
                        <li><a class=link href="/reservation">"Réservation"</a></li>
                        <li><a class=link href="/contact">"Contact"</a></li>
                        <li><a class=link href="/gallery">"Galerie"</a></li>
                        <li><a class=link href="/about">"À propos de nous"</a></li>
                        <li><a class=link href="/policy">"Mentions légales et"<br />"Politique de Confidentialité"</a></li>
                    </ul>
                </nav>
                <div>
                    <h2 class=heading>"Trouvez-nous"</h2>
                    <p class="font-bold">"Adresse :"</p>
                    <p inner_html=address_formatted></p>
                    <p class=label>"Téléphone / Email :"</p>
                    <p><a class=link href=phone_href>{phone_formatted}</a></p>
                    <p><a class=link href=email_href>{email}</a></p>
                </div>
                <div>
                    <h2 class=heading>"Social"</h2>
                    <p class="font-bold">"Retrouvez-nous sur les réseaux sociaux"</p>
                    <p>
                        <a class=link href=INSTAGRAM target="_blank" rel="noreferrer noopener">"instagram"</a>
                        " / "
                        <a class=link href=FACEBOOK target="_blank" rel="noreferrer noopener">"facebook"</a>
                        " / "
                        <a class=link href=MESSENGER target="_blank" rel="noreferrer noopener">"messenger"</a>
                    </p>
                    <p class=label>"Laissez-nous une note"</p>
                    <p>
                        <a class=link href=GOOGLE target="_blank" rel="noreferrer noopener">"google"</a>
                        " / "
                        <a class=link href=TRIPADVISOR target="_blank" rel="noreferrer noopener">"tripadvisor"</a>
                    </p>
                </div>
            </div>
        </footer>
    }
}

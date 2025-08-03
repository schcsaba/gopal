use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <main class="container mx-auto">
            <section class="pt-20">
                <div class="flex flex-wrap md:flex-nowrap w-full p-2">
                    <div class="w-full lg:w-1/2 px-8 mt-12 lg:mt-0">
                        <div class="w-full h-64 md:h-72 lg:h-full">
                            <iframe
                                src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2700.13273053803!2d0.6773934768938756!3d47.4093521711725!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47fcd5dbe973d05f%3A0x4623a58782228f72!2sGopal!5e0!3m2!1sen!2sfr!4v1723976237764!5m2!1sen!2sfr"
                                class="w-full h-full rounded-lg"
                                style="border:0;"
                                allowfullscreen=""
                                loading="lazy"
                                referrerpolicy="no-referrer-when-downgrade"
                                title="Google Map"
                            >
                            </iframe>
                        </div>
                    </div>
                    <div class="flex flex-col p-2 w-full lg:w-1/2">
                        <h2 class="font-sans font-bold text-2xl pb-4 uppercase tracking-wide">"Heures d'ouverture :"</h2>
                        <p class="text-xl pb-2">Mardi : 12h à 14h</p>
                        <p class="text-xl pb-2">Mercredi : 12h à 14h</p>
                        <p class="text-xl pb-2">Jeudi : 12h à 14h</p>
                        <p class="text-xl pb-2">Vendredi : 12h à 14h et 19h à 21h30</p>
                        <p class="text-xl pb-6">Samedi : 12h à 14h</p>
                        <h2 class="font-sans font-bold text-2xl pb-4 uppercase tracking-wide">Coordonnées :</h2>
                        <p class="text-xl pb-2">8 Avenue du Mans, 37100 Tours, France</p>
                        <p class="text-xl pb-2">Tél : <span class="underline decoration-1 underline-offset-2"><a href="tel:+33783654565">07 83 65 45 65</a></span></p>
                        <p class="text-xl pb-2">Email : <span class="underline decoration-1 underline-offset-2"><a href="mailto:contact@legopal.fr">contact@legopal.fr</a></span></p>
                        <p class="text-xl pb-6">Messenger : <span class="underline decoration-1 underline-offset-2"><a href="https://m.me/Legopaltours" target="_blank" rel="noreferrer">LeGopaltours</a></span></p>
                        <p class="text-xl pb-6">"Vous pouvez effectuer un zoom avant et arrière sur la carte à l'aide des boutons situés dans le coin inférieur droit ou avec deux doigts sur les appareils à écran tactile."</p>
                        <h2 class="font-sans font-bold text-2xl pb-4 uppercase tracking-wide">Approche par les transports publics :</h2>
                        <p class="text-xl pb-2">"Le restaurant est à 180m de l'arrêt de tram Tranchée - Ligne A."</p>
                        <p class="text-xl pb-2">Autres lignes urbaines avec un arrêt à proximité : 10, 14, 17, 18, C2, 51, 52, 73</p>
                        <p class="text-xl pb-2">"Plus d'informations sur "<span class="underline decoration-1 underline-offset-2"><a href="https://www.filbleu.fr/" target="_blank" rel="noreferrer">Fil Bleu</a></span>.</p>
                        <p class="text-xl pb-2">Vous trouverez des informations sur le parking sur <span class="underline decoration-1 underline-offset-2"><a href="https://www.tours.fr/page-portail-ma-mairie/services-pratiques/deplacement/stationnement/" target="_blank" rel="noreferrer">le site de la Ville de Tours</a></span>.</p>
                        <p class="text-xl pb-2">Pour toute question ou réservation appelez nous au : <br /><span class="underline decoration-1 underline-offset-2"><a href="tel:+33783654565">07 83 65 45 65</a></span></p>
                    </div>
                </div>
            </section>
            <div class="flex w-full h-64 items-center justify-center">
                <p class="text-5xl">À bientôt</p>
            </div>
        </main>
    }
}

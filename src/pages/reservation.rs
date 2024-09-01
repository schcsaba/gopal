use leptos::*;
use leptos_image::Image;
use leptos_meta::Html;

#[component]
pub fn Reservation() -> impl IntoView {
    view! {
        <main class="container mx-auto">
            <Html lang="fr" />
            <section class="container mx-auto my-20">
                <div class="text-center">
                    <h2 class="font-sans font-bold text-4xl pb-6 uppercase tracking-wide">Sur place ou à emporter ou livraison à domicile</h2>
                    <p class="text-2xl pb-4">Livraison à domicile : <span class="underline decoration-1 underline-offset-2"><a href="https://www.frerestoque.fr/shop/angers-centre-ville/gopal" target="_blank" rel="noreferrer">Cliquez ici</a></span></p>
                    <p class="text-2xl pb-4">Tél : <span class="underline decoration-1 underline-offset-2"><a href="tel:+33783654565">07 83 65 45 65</a></span></p>
                    <p class="text-2xl pb-4">Email : <span class="underline decoration-1 underline-offset-2"><a href="mailto:contact@legopal.fr">contact@legopal.fr</a></span></p>
                    <p class="text-2xl pb-6">Messenger : <span class="underline decoration-1 underline-offset-2"><a href="https://m.me/Legopaltours" target="_blank" rel="noreferrer">LeGopaltours</a></span></p>
                    <p class="text-xl pb-6">"Méthodes de paiement : Espèces · Visa · Mastercard · Ticket restaurant"</p>
                    <p class="text-2xl pb-6 font-extrabold">Tous nos plats sont faits maison.</p>
                    <h2 class="font-sans font-bold text-4xl pt-6 pb-6 uppercase tracking-wide">Réservez une table</h2>
                    <iframe src="https://widget.thefork.com/60d93c62-f1bb-412e-9206-93ab3a1900cb" allow="payment *" title="TheFork" style="width: 100%; min-height: 800px; border: none; overflow: scroll;"></iframe>
                </div>
                <div class="text-center my-10">
                    <h2 class="font-sans font-bold text-3xl pb-6 uppercase tracking-wide">"Heures d'ouverture : "</h2>
                    <div class="grid grid-cols-1 md:grid-cols-4 lg:grid-cols-5 gap-y-6 text-center">
                        <div>
                            <p class="font-sans font-bold text-2xl pb-2">
                            Mardi
                            </p>
                            <p class="text-xl">12h à 14h</p>
                        </div>
                        <div>
                            <p class="font-sans font-bold text-2xl pb-2">
                            Mercredi
                            </p>
                            <p class="text-xl">12h à 14h et 19h à 21h30</p>
                        </div>
                        <div>
                            <p class="font-sans font-bold text-2xl pb-2">
                            Jeudi
                            </p>
                            <p class="text-xl">12h à 14h</p>
                        </div>
                        <div>
                            <p class="font-sans font-bold text-2xl pb-2">
                            Vendredi
                            </p>
                            <p class="text-xl">12h à 14h et 19h à 21h30</p>
                        </div>
                        <div>
                            <p class="font-sans font-bold text-2xl pb-2">
                            Samedi
                            </p>
                            <p class="text-xl">12h à 14h</p>
                        </div>
                    </div>
                </div>
                <div class="flex items-center justify-center pb-8">
                    <img alt="Table de pique-nique" class="h-32" src="assets/img/picnic.svg" />
                </div>
                <div class="text-center">
                    <h2 class="font-sans font-bold text-3xl pb-6 uppercase tracking-wide">"Nos cartes cadeaux !!! 🎁😄"</h2>
                    <p class="text-xl pb-6">En vente au Gopal !</p>
                    <p class="text-xl pb-6">Faites plaisir à coup sûr !</p>
                </div>
                <div class="flex items-center justify-center">
                    <div class="h-128">
                        <Image
                            src="assets/img/giftcard.jpg"
                            blur=true
                            width=827
                            height=591
                            quality=85
                            alt="Carte cadeau"
                        />
                    </div>
                </div>
            </section>
        </main>
    }
}

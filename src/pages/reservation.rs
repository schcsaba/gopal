use leptos::*;

#[component]
pub fn Reservation() -> impl IntoView {
    view! {
        <main class="container mx-auto">
            <section class="container mx-auto my-20">
                <div class="text-center">
                    <h2 class="font-sans font-bold text-4xl pb-6 uppercase tracking-wide">Sur place ou à emporter ou livraison à domicile</h2>
                    <p class="text-2xl pb-4">Livraison à domicile : <a href="https://www.frerestoque.fr/shop/angers-centre-ville/gopal" target="_blank" rel="noreferrer">Cliquez ici</a></p>
                    <p class="text-2xl pb-4">Tél : <a href="tel:+33783654565">07 83 65 45 65</a></p>
                    <p class="text-2xl pb-4">Email : <a href="mailto:contact@legopal.fr">contact@legopal.fr</a></p>
                    <p class="text-2xl pb-6">Messenger : <a href="https://m.me/Legopaltours" target="_blank" rel="noreferrer">LeGopaltours</a></p>
                    <p class="text-xl pb-6">"Méthodes de paiement : Espèces · Visa · Mastercard · Ticket restaurant"</p>
                    <p class="text-2xl pb-6 font-extrabold">Tous nos plats sont faits maison.</p>
                    <h2 class="font-sans font-bold text-4xl pt-6 pb-6 uppercase tracking-wide">Réservez une table</h2>
                    <iframe src="https://widget.thefork.com/60d93c62-f1bb-412e-9206-93ab3a1900cb" allow="payment *" title="TheFork" style="width: 100%; min-height: 800px; border: none; overflow: scroll;"></iframe>
                </div>
                <div class="text-center my-10">
                    <h2 class="font-sans font-bold text-3xl pb-6 uppercase tracking-wide">"Heures d'ouverture : "</h2>
                    <div class="flex items-center justify-evenly text-center">
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
                <div class="flex items-center justify-center">
                    <img class="h-32" src="assets/img/Capturepicpic.png" />
                </div>
            </section>
        </main>
    }
}

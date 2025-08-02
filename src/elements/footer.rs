use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="container mx-auto">
            <div class="flex flex-wrap-reverse lg:flex-nowrap items-center justify-center lg:justify-between">
                <div class="flex justify-center items-center w-full lg:w-1/4">
                    <a href="/"><img alt="Gopal Logo" class="h-24 md:h-40" src="/assets/img/gopal-logo-black.svg" /></a>
                </div>
                <div class="flex justify-center h-72 w-full lg:w-1/4">
                    <ul>
                        <li class="font-sans text-2xl font-bold tracking-wide pb-6 uppercase">Plan du site</li>
                        <li class="pb-1"><a href="/">Accueil</a></li>
                        <li class="pb-1"><a href="/menu">Menu</a></li>
                        <li class="pb-1"><a href="/reservation">Réservation</a></li>
                        <li class="pb-1"><a href="/contact">Contact</a></li>
                        <li class="pb-1"><a href="/gallery">Galerie</a></li>
                        <li class="pb-1"><a href="/about">À propos de nous</a></li>
                        <li class="pb-1"><a href="/policy">Mentions légales et<br />Politique de Confidentialité</a></li>
                    </ul>
                </div>
                <div class="flex justify-center h-64 w-full lg:w-1/4">
                    <ul>
                        <li class="font-sans text-2xl font-bold tracking-wide pb-4 uppercase">Trouvez-nous</li>
                        <li class="font-sans  font-bold text-lg py-2">Adresse :</li>
                        <li>8 Avenue du Mans<br />37100 Tours, France</li>
                        <li class="font-sans  font-bold text-lg py-2">Téléphone / Email :</li>
                        <li><a href="tel:+33783654565">07 83 65 45 65</a></li>
                        <li><a href="mailto:contact@legopal.fr">contact@legopal.fr</a></li>
                    </ul>
                </div>
                <div class="flex justify-center h-64 w-full ml-10 lg:-ml-0 lg:w-1/4">
                    <ul>
                        <li class="font-sans text-2xl font-bold tracking-wide pb-4 uppercase">Social</li>
                        <li class="font-sans font-bold text-lg py-2">Retrouvez-nous sur les réseaux sociaux</li>
                        <li>
                            <a href="https://www.instagram.com/le_gopal/" target="_blank" rel="noreferrer noopener">instagram</a> /
                            <a href="https://www.facebook.com/Legopaltours" target="_blank" rel="noreferrer noopener">facebook</a> /
                            <a href="https://m.me/Legopaltours" target="_blank" rel="noreferrer noopener">messenger</a>
                        </li>
                        <li class="font-sans font-bold text-lg py-2">Laissez-nous une note</li>
                        <li>
                            <a href="https://www.google.com/search?hl=fr-FR&gl=fr&q=Gopal,+8+Av.+du+Mans,+37100+Tours" target="_blank" rel="noreferrer noopener">google</a> /
                            <a href="https://www.tripadvisor.fr/Restaurant_Review-g187130-d19250495-Reviews-Gopal-Tours_Indre_et_Loire_Centre_Val_de_Loire.html" target="_blank" rel="noreferrer noopener">tripadvisor</a>
                        </li>
                    </ul>
                </div>
            </div>
        </footer>
    }
}

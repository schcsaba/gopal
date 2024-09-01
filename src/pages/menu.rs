use leptos::*;
use leptos_meta::Html;

#[component]
pub fn Menu() -> impl IntoView {
    view! {
        <main class="container mx-auto">
            <Html lang="fr" />
            <section class="mt-20 text-center">
                <h2 class="text-4xl tracking-widest uppercase pb-5">Notre carte</h2>
                <div class="flex justify-center items-center">
                    <img alt="Etagère avec ustensiles de cuisine" src="assets/img/detourCapture.svg" />
                </div>

                <ul>
                    <li class="pt-10 pb-6 text-5xl tracking-widest uppercase">Nos Formules</li>
                    <li class="pb-20 text-2xl">
                        <ul>
                            <li class="py-4 font-sans font-extrabold">"Assiette Gourmande 9,00 €"</li>
                            <li class="mb-8">samosa aux légumes + salade composée</li>
                            <li class="py-4 font-sans font-extrabold">"Plat du jour 13,00 €"</li>
                            <li class="mb-8">riz parfumé + curry <span class="font-extrabold">ou</span> boulettes de légumes <span class="font-extrabold">ou</span> soupe (au choix) + galette papadam</li>
                            <li class="py-4 font-sans font-extrabold">"Menu « Gopal » 16,00 €"</li>
                            <li class="mb-8">salade composée + riz parfumé + curry + boulettes de légumes + soupe + galette papadam</li>
                            <li class="py-4 font-sans font-extrabold">"Spéciale du chef 14,00 €"</li>
                            <li>lasagne royale, calzoni, fougasse, etc... + salade composée</li>
                        </ul>
                    </li>
                    <li class="pt-10 pb-6 text-5xl tracking-widest uppercase">Nos Desserts</li>
                    <li class="pb-20 text-2xl">
                        <ul>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Cheesecake (caramel/chocolat, mangue ou fruits rouges...) 6,50 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Cupcake (ex : chocolat, crème de marron, framboise...) 5,80 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Crumble (pomme cannelle) 6,50 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Mousse choco vegan 5,80 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Pannacotta vegan 5,80 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Gâteau du chef vegan (la part) 6,50 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Halava (gâteau semoule) 5,80 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Tarte au citron vegan (la part) 6,50 €"</li>
                        </ul>
                    </li>
                    <li class="pt-10 pb-6 text-5xl tracking-widest uppercase">Nos Boissons</li>
                    <li class="pb-20 text-2xl">
                        <ul>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Lassi (au yaourt vegan, parfum mangue ou rose) 25 cl 4,00 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Limonade maison (menthe fraîche, citron vert, gingembre) 25 cl 3,50 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"San Pellegrino aromatisée 33 cl 2,00 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Bière sans alcool 25 cl 3,00 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Tisane 20 cl 2,00 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Eau pétillante 33 cl 2,00 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Eau plate 50 cl 2,00 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Café expresso 2,50 €"</li>
                            <li class="py-4 mb-4 font-sans font-extrabold">"Double expresso 3,00 €"</li>
                        </ul>
                    </li>
                </ul>
            </section>
        </main>
    }
}

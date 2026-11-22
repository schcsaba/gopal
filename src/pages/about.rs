use crate::pages::menu::Swash;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <main id="contenu" class="bg-papier">
            <section class="wrap py-12 lg:py-20">
                <div class="mx-auto flex max-w-[46rem] flex-col gap-6">
                <h1 class="pb-2 font-serif text-5xl leading-none font-normal tracking-[-0.02em] sm:text-6xl lg:text-8xl">"À propos de " <em class="text-piment">"nous"</em></h1>
                <Swash class="w-56 lg:w-72" />
                <p class="pt-4 font-serif text-2xl leading-snug lg:text-[1.875rem]">
                    "Le restaurant Le Gopal, situé en haut de l’avenue de la Tranchée à Tours, est né de la passion de Sylvain et Sumitra
                    pour la cuisine végétarienne et végane, inspirée à la fois par les traditions indiennes et les saveurs internationales.
                    Depuis 2019, ils vous invitent à découvrir un univers culinaire où l’éthique et le respect de l’environnement sont au cœur
                    de chaque plat."
                </p>
                <p class="text-xl leading-relaxed text-mousse lg:text-[1.375rem]">
                    "Sylvain et Sumitra ont commencé leur aventure dans la restauration dès 1998, en proposant des plats végétariens indiens
                    dans des salons bio et bien-être à travers la France. Leur rêve d’ouvrir un restaurant fixe s’est concrétisé grâce à une
                    opportunité à Tours et à la rencontre de Radha, leur cheffe cuisinière. Avec son parcours international, elle apporte
                    à chaque assiette une authenticité unique et une richesse de saveurs."
                </p>
                <p class="text-xl leading-relaxed text-mousse lg:text-[1.375rem]">
                    "Au Gopal, l’engagement pour une alimentation plus respectueuse va de pair avec l’utilisation de produits locaux et de
                    qualité. Les légumes proviennent de maraîchers de la région, et le lait est fourni par un fermier local. Pour Sylvain et
                    Sumitra, le choix du végétarien et du végan n’est pas une tendance, mais une conviction profonde d’une alimentation plus
                    harmonieuse pour le corps, l’esprit et la planète."
                </p>
                <p class="text-xl leading-relaxed text-mousse lg:text-[1.375rem]">
                    "Malgré les défis des dernières années, notamment les périodes difficiles de 2020 et 2021, Le Gopal a su trouver sa place
                    parmi les adresses appréciées des tourangeaux, qui soutiennent fidèlement ce projet chaleureux et authentique."
                </p>
                <p class="text-xl leading-relaxed text-mousse lg:text-[1.375rem]">
                    "Prêts à découvrir ou redécouvrir Le Gopal ? Sylvain, Sumitra et toute l’équipe vous attendent avec impatience pour partager
                    leur passion dans une ambiance conviviale et pleine de bonnes vibrations."
                </p>
                </div>
                <div class="frame frame-br mx-auto mt-14 max-w-[40rem]">
                    <img class="block h-auto w-full" width=597 height=397 alt="Les fondateurs" src="/assets/img/founders.webp" />
                </div>
            </section>
            <section class="border-t border-ligne">
                <div class="wrap py-16">
                    <iframe class="h-screen w-full" src="https://81e3c7a639fc4faf942c0dbec787942f.elf.site" title="Google Reviews"></iframe>
                </div>
            </section>
        </main>
    }
}

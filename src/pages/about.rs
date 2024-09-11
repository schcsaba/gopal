use leptos::*;
use leptos_image::provide_image_context;
use leptos_image::Image;
use leptos_meta::Html;

#[component]
pub fn About() -> impl IntoView {
    provide_image_context();
    view! {
        <main class="container mx-auto">
            <Html lang="fr" />
            <section class="mt-20">
                <h2 class="font-sans font-bold text-4xl pb-8 uppercase tracking-wide text-center">À propos de nous</h2>
                <p class="text-2xl pb-4">
                    "Le restaurant Le Gopal, situé en haut de l’avenue de la Tranchée à Tours, est né de la passion de Sylvain et Sumitra
                    pour la cuisine végétarienne et végane, inspirée à la fois par les traditions indiennes et les saveurs internationales.
                    Depuis 2019, ils vous invitent à découvrir un univers culinaire où l’éthique et le respect de l’environnement sont au cœur
                    de chaque plat."
                </p>
                <p class="text-2xl pb-4">
                    "Sylvain et Sumitra ont commencé leur aventure dans la restauration dès 1998, en proposant des plats végétariens indiens
                    dans des salons bio et bien-être à travers la France. Leur rêve d’ouvrir un restaurant fixe s’est concrétisé grâce à une
                    opportunité à Tours et à la rencontre de Radha, leur cheffe cuisinière. Avec son parcours international, elle apporte
                    à chaque assiette une authenticité unique et une richesse de saveurs."
                </p>
                <p class="text-2xl pb-4">
                    "Au Gopal, l’engagement pour une alimentation plus respectueuse va de pair avec l’utilisation de produits locaux et de
                    qualité. Les légumes proviennent de maraîchers de la région, et le lait est fourni par un fermier local. Pour Sylvain et
                    Sumitra, le choix du végétarien et du végan n’est pas une tendance, mais une conviction profonde d’une alimentation plus
                    harmonieuse pour le corps, l’esprit et la planète."
                </p>
                <p class="text-2xl pb-4">
                    "Malgré les défis des dernières années, notamment les périodes difficiles de 2020 et 2021, Le Gopal a su trouver sa place
                    parmi les adresses appréciées des tourangeaux, qui soutiennent fidèlement ce projet chaleureux et authentique."
                </p>
                <p class="text-2xl pb-6">
                    "Prêts à découvrir ou redécouvrir Le Gopal ? Sylvain, Sumitra et toute l’équipe vous attendent avec impatience pour partager
                    leur passion dans une ambiance conviviale et pleine de bonnes vibrations."
                </p>
                <div class="flex items-center justify-center">
                    <div class="h-128 pb-6">
                        <Image
                            src="/assets/img/founders.png"
                            blur=true
                            width=597
                            height=397
                            quality=85
                            alt="Les fondateurs"
                        />
                    </div>
                </div>
                <iframe class="w-full h-screen pb-6" src="https://81e3c7a639fc4faf942c0dbec787942f.elf.site" title="Google Reviews"></iframe>
            </section>
        </main>
    }
}

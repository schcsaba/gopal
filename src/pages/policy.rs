use leptos::*;
use leptos_meta::Html;

#[component]
pub fn Policy() -> impl IntoView {
    view! {
            <main class="container mx-auto">
                <Html lang="fr" />
                <section class="py-20">
                    <div class="p-5">
                        <h2 class="text-4xl tracking-widest uppercase pb-5">Mentions légales et politique de confidentialité</h2>
                        <p class="text-2xl mb-12">
                            En navigant sur le site legopal.fr, vous reconnaissez avoir pris connaissance de ces mentions légales
                            et vous engagez à les respecter.
                        </p>
                        <h3  class="text-3xl tracking-widest uppercase pb-5">Edition du site</h3>
                        <p class="text-2xl mb-12">
                            "Le site legopal.fr est édité par l'association Gopal (Adresse : 8 Avenue du Mans 37100 Tours,
                            SIRET/APE: 85165504300012/9499Z)."
                        </p>
                        <h3  class="text-3xl tracking-widest uppercase pb-5">Propriété intellectuelle</h3>
                        <p class="text-2xl mb-12">
                            "Tous les contenus présents, tels que les textes, graphiques, logos, images, photographies,
                            vidéos présents sur ce site sont, sauf mention contraire, la propriété de l'association Gopal."
                        </p>
                        <h3  class="text-3xl tracking-widest uppercase pb-5">Cookies</h3>
                        <p class="text-2xl mb-4">
                            "Un ou plusieurs « cookies » peuvent être placés sur le disque dur de l'ordinateur des internautes
                            visiteurs du site afin de faciliter leur connexion au site, la gestion des comptes ou de garder en mémoire
                            leurs réglages. Ces cookies peuvent aussi nous permettre de mieux comprendre ce qui vous intéresse sur notre
                            site et de vous proposer du contenu pertinent."
                        </p>
                        <p class="text-2xl mb-4">
                            "Les informations fournies sur le site legopal.fr le sont à titre indicatif."
                        </p>
                        <p class="text-2xl mb-4">
                            "L'exactitude, la complétude, l'actualité des informations diffusées sur legopal.fr ne sauraient être garanties."
                        </p>
                        <p class="text-2xl mb-4">
                            "Vous êtes le seul et unique responsable de l'usage du site legopal.fr et de ses contenus."
                        </p>
                        <p class="text-2xl mb-4">
                            "Le site legopal.fr ne pourra être tenu pour responsable d'un usage non conforme aux normes des lois en vigueur,
                            du site internet ou de ses contenus."
                        </p>
                        <p class="text-2xl mb-4">
                            "Ce site peut comporter des informations fournies par des sociétés externes ou des liens hypertextes vers d'autres
                            sites qui ne sont pas gérés par legopal.fr."
                        </p>
                        <p class="text-2xl mb-4">
                            "L'existence d'un lien depuis legopal.fr vers un autre site ne constitue pas une validation de ce site ou de son contenu."
                        </p>
                        <p class="text-2xl mb-4">
                            "Tout message publié, de manière générale, demeure sous l'entière responsabilité de son auteur."
                        </p>
                    </div>
                </section>
            </main>
    }
}

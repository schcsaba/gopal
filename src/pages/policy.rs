use leptos::prelude::*;
use crate::data::contact::get_contact_info;

#[component]
pub fn Policy() -> impl IntoView {
    let contact = get_contact_info();
    view! {
            <main id="contenu" class="bg-papier">
                <section class="wrap py-12 lg:py-20">
                    <div class="mx-auto max-w-[46rem]">
                        <h1 class="pb-8 font-serif text-4xl leading-tight font-normal tracking-[-0.01em] sm:text-5xl lg:text-6xl">Mentions légales et politique de confidentialité</h1>
                        <p class="mb-10 text-xl leading-relaxed text-mousse">
                            En navigant sur le site legopal.fr, vous reconnaissez avoir pris connaissance de ces mentions légales
                            et vous engagez à les respecter.
                        </p>
                        <h2 class="pt-4 pb-3 font-serif text-2xl font-medium lg:text-3xl">Edition du site</h2>
                        <p class="mb-10 text-xl leading-relaxed text-mousse">
                            {format!("Le site legopal.fr est édité par l'association Gopal (Adresse : {},
                            SIRET/APE: 85165504300012/9499Z).", contact.address)}
                        </p>
                        <h2 class="pt-4 pb-3 font-serif text-2xl font-medium lg:text-3xl">Propriété intellectuelle</h2>
                        <p class="mb-10 text-xl leading-relaxed text-mousse">
                            "Tous les contenus présents, tels que les textes, graphiques, logos, images, photographies,
                            vidéos présents sur ce site sont, sauf mention contraire, la propriété de l'association Gopal."
                        </p>
                        <h2 class="pt-4 pb-3 font-serif text-2xl font-medium lg:text-3xl">Cookies</h2>
                        <p class="mb-4 text-xl leading-relaxed text-mousse">
                            "Un ou plusieurs « cookies » peuvent être placés sur le disque dur de l'ordinateur des internautes
                            visiteurs du site afin de faciliter leur connexion au site, la gestion des comptes ou de garder en mémoire
                            leurs réglages. Ces cookies peuvent aussi nous permettre de mieux comprendre ce qui vous intéresse sur notre
                            site et de vous proposer du contenu pertinent."
                        </p>
                        <p class="mb-4 text-xl leading-relaxed text-mousse">
                            "Les informations fournies sur le site legopal.fr le sont à titre indicatif."
                        </p>
                        <p class="mb-4 text-xl leading-relaxed text-mousse">
                            "L'exactitude, la complétude, l'actualité des informations diffusées sur legopal.fr ne sauraient être garanties."
                        </p>
                        <p class="mb-4 text-xl leading-relaxed text-mousse">
                            "Vous êtes le seul et unique responsable de l'usage du site legopal.fr et de ses contenus."
                        </p>
                        <p class="mb-4 text-xl leading-relaxed text-mousse">
                            "Le site legopal.fr ne pourra être tenu pour responsable d'un usage non conforme aux normes des lois en vigueur,
                            du site internet ou de ses contenus."
                        </p>
                        <p class="mb-4 text-xl leading-relaxed text-mousse">
                            "Ce site peut comporter des informations fournies par des sociétés externes ou des liens hypertextes vers d'autres
                            sites qui ne sont pas gérés par legopal.fr."
                        </p>
                        <p class="mb-4 text-xl leading-relaxed text-mousse">
                            "L'existence d'un lien depuis legopal.fr vers un autre site ne constitue pas une validation de ce site ou de son contenu."
                        </p>
                        <p class="mb-4 text-xl leading-relaxed text-mousse">
                            "Tout message publié, de manière générale, demeure sous l'entière responsabilité de son auteur."
                        </p>
                    </div>
                </section>
            </main>
    }
}

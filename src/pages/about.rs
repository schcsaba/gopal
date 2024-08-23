use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <main class="container mx-auto">
            <section class="mt-20 text-center">
                <h2 class="font-sans font-bold text-4xl pb-6 uppercase tracking-wide">À propos de nous</h2>
                <p class="text-2xl pb-2">
                    Créé à Tours en Novembre 2019, le Gopal vous propose de venir déguster une cuisine
                    100% végétarienne et végane dans un cadre calme et serein.
                </p>
                <p class="text-2xl pb-6">
                    "Dans une démarche écologique, nous nous efforçons d’utiliser au maximum des produits frais
                    et de production locale... et tous nos plats et desserts sont faits maison !"
                </p>
                <iframe class="pb-6" src="https://81e3c7a639fc4faf942c0dbec787942f.elf.site" width="100%" height="1000" frameborder="0"></iframe>
            </section>
        </main>
    }
}

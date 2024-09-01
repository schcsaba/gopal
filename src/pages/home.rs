use leptos::*;
use leptos_image::Image;
use leptos_meta::Html;

#[component]
pub fn Home() -> impl IntoView {
    view! {
            <main class="container mx-auto">
                <Html lang="fr" />
                <section class="py-10">
                    <div class="flex flex-wrap md:flex-nowrap p-5">
                        <div class="flex items-center justify-center p-5 mx-auto w-full lg:w-1/2">
                            <div class="object-fill">
                                <Image
                                    src="assets/img/menu.jpg"
                                    blur=true
                                    width=580
                                    height=480
                                    quality=85
                                    alt="Découvrez notre menu"
                                />
                            </div>
                        </div>
                        <div class="flex flex-col p-5 items-center text-center justify-center w-full lg:w-1/2">
                            <h2 class="text-4xl tracking-widest uppercase pb-5" data-scroll>Découvrez notre menu</h2>
                            <p class="text-2xl mb-12 p-5" data-scroll>
                                Au Gopal venez découvrir notre cuisine végétarienne et végane Internationale,
                                aussi bien inspirée des saveurs indiennes que des bons petits plats de chez nous.
                            </p>
                            <button class="px-3 py-4">
                                <a class="px-3 py-4 bg-black text-white text-nowrap text-2xl font-sans uppercase tracking-widest border hover:text-black hover:bg-white hover:border-black transition duration-300"
                                href="menu">Aller au menu</a>
                            </button>
                        </div>
                    </div>
                </section>
                <section class="py-10">
                    <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-4 divide-y-0">
                        <div class="flex flex-col items-center justify-center mb-4 lg:mb-0">
                            <a href="about">
                                <div class="h-64 m-5">
                                    <Image
                                        src="assets/img/about.jpg"
                                        blur=true
                                        width=270
                                        height=270
                                        quality=85
                                        alt="À propos de nous"
                                    />
                                </div>
                            </a>
                            <h2 class="font-sans text-3xl tracking-widest uppercase mb-2 text-center"><a href="about">À propos de nous</a></h2>
                            <div class="px-20 text-center">
                                <p>
                                    Créé à Tours en Novembre 2019, le Gopal vous propose de venir déguster une cuisine 100% végétarienne
                                    et végane dans un cadre calme et serein.
                                </p>
                            </div>
                        </div>
                        <div class="flex flex-col items-center justify-center mb-4 lg:mb-0">
                            <a href="menu">
                                <div class="h-64 m-5">
                                    <Image
                                        src="assets/img/specialities.jpg"
                                        blur=true
                                        width=270
                                        height=270
                                        quality=85
                                        alt="Spécialités"
                                    />
                                </div>
                            </a>
                            <h2 class="font-sans text-3xl tracking-widest uppercase mb-2"><a href="menu">Spécialités</a></h2>
                            <div class="px-20 text-center">
                                <p>
                                    Découvrez nos spécialités végétariennes et véganes, alliant cuisine traditionnelle et inspirations
                                    internationales, avec des plats savoureux comme notre curry parfumé, nos lasagnes royales, ou encore
                                    nos desserts gourmands.
                                </p>
                            </div>
                        </div>
                        <div class="flex flex-col items-center justify-center mb-4 lg:mb-0">
                            <a href="gallery">
                                <div class="h-64 m-5">
                                    <Image
                                        src="assets/img/gallery.png"
                                        blur=true
                                        width=270
                                        height=270
                                        quality=85
                                        alt="Galerie"
                                    />
                                </div>
                            </a>
                            <h2 class="font-sans text-3xl tracking-widest uppercase mb-2"><a href="gallery">Galerie</a></h2>
                            <div class="px-20 text-center">
                                <p>
                                    "Explorez notre galerie pour un aperçu en images de nos délicieuses préparations culinaires
                                    et de l’atmosphère chaleureuse de notre restaurant."
                                </p>
                            </div>
                        </div>
                        <div class="flex flex-col items-center justify-center mb-4 lg:mb-0">
                            <div class="h-64 m-5">
                                <Image
                                    src="assets/img/events.png"
                                    blur=true
                                    width=270
                                    height=270
                                    quality=85
                                    alt="Evénements"
                                />
                            </div>
                            <h2 class="font-sans text-3xl tracking-widest uppercase mb-2">Evénements</h2>
                            <div class="px-20 text-center">
                                <p>
                                    Participez à nos soirées kirtan chaque vendredi de 19h00 à 20h30, et laissez-vous emporter par une
                                    méditation musicale apaisante dans une ambiance conviviale.
                                </p>
                            </div>
                        </div>
                    </div>
                </section>
                <section class="py-10">
                    <div class="flex flex-wrap md:flex-nowrap w-full p-5">
                        <div class="flex flex-col p-5 items-center text-center justify-center w-full lg:w-1/2">
                            <h2 class="text-4xl tracking-widest uppercase pb-5">Que des bonnes vibrations</h2>
                            <p class="text-2xl mb-12 text-center">
                                Venez partager des moments de convivialité dans une ambiance joyeuse et positive, où bonne humeur et bien-être sont toujours au rendez-vous.
                            </p>
                        </div>
                        <div class="flex items-center justify-center p-5 mx-auto w-full lg:w-1/2">
                            <div class="object-fill">
                                <Image
                                    src="assets/img/good_vibes.jpg"
                                    blur=true
                                    width=580
                                    height=480
                                    quality=85
                                    alt="Que des bonnes vibration"
                                />
                            </div>
                        </div>
                    </div>
                </section>
                <section
                    class="my-10 bg-fixed bg-center object-contain overflow-x-visible w-full"
                    style="background-image: url('assets/img/affection.webp')"
                >
                    <div class="container px-4 mx-auto">
                        <div class="pt-20 pb-24 px-8 md:px-16 rounded">
                            <div class="flex flex-wrap -mx-8">
                                <div class="w-full lg:w-1/2 px-8 flex">
                                    <div class="my-auto">
                                        <h2 class="text-3xl lg:text-4xl font-semibold text-white uppercase">Rendez-nous visite :</h2>
                                        <p class="mt-2 leading-loose font-sans uppercase tracking-wide font-bold text-white">Le Gopal</p>
                                        <p class="mt-2 mb-6 leading-loose text-white">8 Avenue du Mans<br />37100 Tours, France</p>
                                        <h2 class="text-3xl lg:text-4xl font-semibold text-white uppercase">Appelez-nous :</h2>
                                        <p class="mt-2 mb-6 leading-loose text-white">
                                            <a href="tel:+33783654565">07 83 65 45 65</a>
                                        </p>
                                        <a
                                            class="inline-block py-4 px-8 text-xs border bg-black text-white uppercase tracking-widest font-semibold leading-none hover:bg-white hover:border-white hover:text-black rounded-none transition duration-300"
                                            href="reservation">Réservation
                                        </a>
                                    </div>
                                </div>
                                <div class="w-full lg:w-1/2 px-8 mt-12 lg:mt-0">
                                    <div class="aspect-w-4 aspect-h-3">
                                        <iframe
                                            src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2700.13273053803!2d0.6773934768938756!3d47.4093521711725!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47fcd5dbe973d05f%3A0x4623a58782228f72!2sGopal!5e0!3m2!1sen!2sfr!4v1723976237764!5m2!1sen!2sfr"
                                            width="600"
                                            height="450"
                                            style="border:0;"
                                            allowfullscreen=""
                                            loading="lazy"
                                            referrerpolicy="no-referrer-when-downgrade"
                                            title="Google Map"
                                        >
                                        </iframe>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
                <iframe class="pb-6" src="https://2bc10f5dd2994533866aac1fe6e8e6fb.elf.site" width="100%" height="700" frameborder="0" title="Google Reviews"></iframe>
            </main>
    }
}

use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
            <main class="container mx-auto">
                <section class="py-20">
                    <div class="flex flex-wrap md:flex-nowrap p-5">
                        <div class="flex items-center justify-center p-5 mx-auto w-full lg:w-1/2">
                            <img alt="Menu Link Image" class="object-fill" src="assets/img/menu.png" />
                        </div>
                        <div class="flex flex-col p-5 items-center text-center justify-center w-full lg:w-1/2">
                            <h2 class="text-4xl tracking-widest uppercase pb-5" data-scroll>Découvrez notre menu</h2>
                            <p class="text-2xl mb-12 p-5" data-scroll>
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus ut
                                iaculis lectus. In lobortis tortor eget venenatis ultrices. Fusce rhoncus tincidunt purus et egestas.
                                Maecenas erat nisl, porta nec.
                            </p>
                            <button class="px-3 py-4">
                                <a class="px-3 py-4 bg-black text-white font-sans uppercase tracking-widest border hover:text-black hover:bg-white hover:border-black transition duration-300"
                                href="menu">Aller au menu</a>
                            </button>
                        </div>
                    </div>
                </section>
                <section class="py-20">
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 divide-y-0">
                        <div class="flex flex-col items-center justify-center mb-4 lg:mb-0">
                            <img class="h-64 m-5" src="assets/img/rezervation.png" />
                            <h2 class="font-sans text-3xl tracking-widest uppercase mb-2">Evénements</h2>
                            <div class="px-20 text-center">
                                <p>
                                    Maecenas erat nisl, porta nec fringilla in, tincidunt eget sem. Nunc in magna vitae lectus ultrices vestibulum. Maecenas rutrum rhoncus ipsum, in porta lectus viverra eget
                                </p>
                            </div>
                        </div>
                        <div class="flex flex-col items-center justify-center mb-4 lg:mb-0">
                            <img class="h-64 m-5" src="assets/img/rezervation.png" />
                            <h2 class="font-sans text-3xl tracking-widest uppercase mb-2 text-center">À propos de nous</h2>
                            <div class="px-20 text-center">
                                <p>
                                    Maecenas erat nisl, porta nec fringilla in, tincidunt eget sem. Nunc in magna vitae lectus ultrices vestibulum. Maecenas rutrum rhoncus ipsum, in porta lectus viverra eget
                                </p>
                            </div>
                        </div>
                        <div class="flex flex-col items-center justify-center mb-4 lg:mb-0">
                            <img class="h-64 m-5" src="assets/img/rezervation.png" />
                            <h2 class="font-sans text-3xl tracking-widest uppercase mb-2">Spécialités</h2>
                            <div class="px-20 text-center">
                                <p>
                                    Maecenas erat nisl, porta nec fringilla in, tincidunt eget sem. Nunc in magna vitae lectus ultrices vestibulum. Maecenas rutrum rhoncus ipsum, in porta lectus viverra eget
                                </p>
                            </div>
                        </div>
                        <div class="flex flex-col items-center justify-center mb-4 lg:mb-0">
                            <img class="h-64 m-5" src="assets/img/rezervation.png" />
                            <h2 class="font-sans text-3xl tracking-widest uppercase mb-2">Galerie</h2>
                            <div class="px-20 text-center">
                                <p>
                                    Maecenas erat nisl, porta nec fringilla in, tincidunt eget sem. Nunc in magna vitae lectus ultrices vestibulum. Maecenas rutrum rhoncus ipsum, in porta lectus viverra eget
                                </p>
                            </div>
                        </div>
                    </div>
                </section>
                <section class="py-20">
                    <div class="flex flex-wrap md:flex-nowrap w-full p-5">
                        <div class="flex flex-col p-5 items-center text-center justify-center w-full lg:w-1/2">
                            <h2 class="text-4xl tracking-widest uppercase pb-5">Que des BONNES vibrations</h2>
                            <p class="text-2xl mb-12 text-center">
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus ut iaculis lectus. In lobortis tortor eget venenatis ultrices. Fusce rhoncus tincidunt purus et egestas. Maecenas erat nisl, porta nec.
                            </p>
                        </div>
                        <div class="flex items-center justify-center p-5 mx-auto w-full lg:w-1/2">
                            <img alt="Menu Link Image" class="object-fill" src="assets/img/good_vibes.jpg" />
                        </div>
                    </div>
                </section>
                <section
                    class="my-20 bg-fixed bg-center object-contain overflow-x-visible w-full"
                    style="background-image: url('assets/img/sunset.jpg')"
                >
                    <div class="container px-4 mx-auto">
                        <div class="pt-20 pb-24 px-8 md:px-16 rounded">
                            <div class="flex flex-wrap -mx-8">
                                <div class="w-full lg:w-1/2 px-8 flex">
                                    <div class="my-auto">
                                        <h2 class="text-3xl lg:text-4xl font-semibold text-white uppercase">Rendez-nous visite :</h2>
                                        <p class="mt-2 leading-loose font-sans uppercase tracking-wide font-bold text-white">LE GOPAL</p>
                                        <p class="mt-2 mb-6 leading-loose text-white">8 Avenue du Mans<br />37100 Tours, France</p>
                                        <h2 class="text-3xl lg:text-4xl font-semibold text-white uppercase">Appelez-nous :</h2>
                                        <p class="mt-2 mb-6 leading-loose text-white">
                                            07 83 65 45 65
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
                                        >
                                        </iframe>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            </main>
    }
}

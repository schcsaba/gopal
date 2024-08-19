use leptos::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <main class="container mx-auto">
            <section class="pt-10">
                <div class="flex items-center justify-center p-5">
                    <ul>
                        <li class="font-sans font-extrabold tracking-widest pb-4 text-3xl lg:text-5xl text-center uppercase">Le Gopal</li>
                        <li class="font-sans font-extrabold tracking-wide pb-4 text-3xl lg:text-5xl text-center">8 Avenue du Mans</li>
                        <li class="font-sans font-extrabold tracking-wide pb-4 text-3xl lg:text-5xl text-center">37100 Tours, France</li>
                        <li class="font-sans font-extrabold tracking-wide pb-4 text-3xl lg:text-5xl text-center">07 83 65 45 65</li>
                        <li class="font-sans font-extrabold tracking-wide pb-4 text-3xl lg:text-5xl text-center">contact@legopal.fr</li>
                    </ul>
                </div>
            </section>
            <section class="pt-20">
                <div class="flex flex-wrap md:flex-nowrap w-full p-5">
                    <div class="flex flex-col p-5 items-center text-center justify-center w-full lg:w-1/2">
                        <h2 class="text-4xl tracking-widest uppercase pb-5">Que des BONNES vibrations</h2>
                        <p class="text-2xl mb-12 text-center">Reach it with la la la bicileta, a lot of parking spots available</p>
                    </div>
                    <div class="flex items-center justify-center p-5 mx-auto w-full lg:w-1/2">
                        <img alt="Menu Link Image" class="object-fill" src="assets/img/contact_page_img.jpg" />
                    </div>
                </div>
            </section>
            <div class="flex w-full h-64 items-center justify-center">
                <p class="text-5xl">À bientôt</p>
            </div>
        </main>
    }
}

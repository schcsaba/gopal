use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="container mx-auto">
            <section class="py-20">
                <div class="flex flex-wrap md:flex-nowrap p-5">
                    <div class="flex flex-col p-5 items-center text-center justify-center w-full lg:w-1/2">
                        <h2 class="text-4xl tracking-widest uppercase pb-5" data-scroll>Discover our menu</h2>
                        <p class="text-2xl mb-12 p-5" data-scroll>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus ut
                            iaculis lectus. In lobortis tortor eget venenatis ultrices. Fusce rhoncus tincidunt purus et egestas.
                            Maecenas erat nisl, porta nec.</p>
                        <button class="px-3 py-4">
                            <a class="px-3 py-4 bg-black text-white font-sans uppercase tracking-widest border hover:text-black hover:bg-white hover:border-black transition duration-300"
                            href="meniu.html">Go to menu</a>
                        </button>
                    </div>
                </div>
            </section>
        </main>
    }
}

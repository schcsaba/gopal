use leptos::*;

#[component]
pub fn Reservation() -> impl IntoView {
    view! {
        <main class="container mx-auto">
            <section class="container mx-auto my-20">
                <div class="text-center">
                    <h2 class="font-sans font-bold text-3xl pb-6 uppercase tracking-wide">Reservation :</h2>
                    <p class="text-3xl pb-4">+40 742 742 642</p>
                    <p class="text-2xl pb-6">or</p>
                    <p class="text-3xl pb-4">info@picnicbistro.ro</p>
                    <h2 class="font-sans font-bold text-3xl pt-6 pb-6 uppercase tracking-wide">House rules :</h2>
                    <p class="text-xl pb-2">Terasa este structurata pe mese de 3 / 6 persoane </p>
                    <p class="text-xl pb-2">Rezervarile se fac pe cat pe posibil, inainte cu o zi, respectiv 3 zile, pentru grupurile
                    mai mare decat 6 persoane.</p>
                    <p class="text-xl pb-2">Rezervarile se pastreaza 20 de min</p>
                </div>
                <div class="text-center my-10">
                    <h2 class="font-sans font-bold text-3xl pb-6 uppercase tracking-wide">Opening Hours : </h2>
                    <div class="flex items-center justify-evenly text-center">
                    <div>
                        <p class="font-sans font-bold text-2xl pb-2">
                        Luni
                        </p>
                        <p class="text-xl"> 11 - 23</p>
                    </div>
                    <div>
                        <p class="font-sans font-bold text-2xl pb-2">
                        Luni
                        </p>
                        <p class="text-xl"> 11 - 23</p>
                    </div>
                    <div>
                        <p class="font-sans font-bold text-2xl pb-2">
                        Luni
                        </p>
                        <p class="text-xl"> 11 - 23</p>
                    </div>
                    <div>
                        <p class="font-sans font-bold text-2xl pb-2">
                        Luni
                        </p>
                        <p class="text-xl"> 11 - 23</p>
                    </div>
                    </div>
                </div>
                <div class="flex items-center justify-center">
                    <img class="h-32" src="assets/img/Capturepicpic.png" />
                </div>
            </section>
        </main>
    }
}

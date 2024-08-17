use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="container mx-auto">
            <div class="flex flex-wrap-reverse lg:flex-nowrap items-center justify-center lg:justify-between">
                <div class="flex justify-center items-center w-full lg:w-1/4">
                    <img alt="Picnic Bistro Logo" class="h-24 md:h-32" src="assets/img/picnic-logo-black.svg" />
                </div>
                <div class="flex justify-center h-64 w-full lg:w-1/4">
                    <ul>
                        <li class="font-sans text-2xl font-bold tracking-wide pb-6">SITE MENU</li>
                        <li class="pb-1"><a href="menu">Menu</a></li>
                        <li class="pb-1"><a href="reservation">Reservation</a></li>
                        <li class="pb-1"><a href="contact">Contact</a></li>
                        <li class="pb-1">Site Policy</li>
                        <li class="pb-1">Cookie Policy</li>
                        <li class="pb-1">GDPR Information</li>
                    </ul>
                </div>
                <div class="flex justify-center h-64 w-full lg:w-1/4">
                    <ul class="-ml-20">
                        <li class="font-sans text-2xl font-bold tracking-wide pb-4">FIND US</li>
                        <li class="font-sans  font-bold text-lg py-2">Address :</li>
                        <li>Ineu, Bihor, RO</li>
                        <li class="font-sans  font-bold text-lg py-2">Phone / Email :</li>
                        <li>+40 742 742 642</li>
                        <li>info@picnicbistro.ro</li>
                    </ul>
                </div>
                <div class="flex justify-center h-64 w-full ml-10 lg:-ml-0 lg:w-1/4">
                    <ul>
                        <li class="font-sans text-2xl font-bold tracking-wide pb-4">SOCIAL</li>
                        <li class="font-sans font-bold text-lg py-2">Find us on social</li>
                        <li>instagram / facebook / soundcloud</li>
                        <li class="font-sans font-bold text-lg py-2">Leave us a note</li>
                        <li>resengo / yelp / tripadvisor</li>
                    </ul>
                </div>
            </div>
        </footer>
    }
}

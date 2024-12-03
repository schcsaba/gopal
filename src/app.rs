use crate::elements::footer::Footer;
use crate::elements::header::Header;
use crate::elements::modal::Modal;
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::about::About;
use crate::pages::contact::Contact;
use crate::pages::gallery::Gallery;
use crate::pages::home::Home;
use crate::pages::menu::Menu;
use crate::pages::policy::Policy;
use crate::pages::reservation::Reservation;
#[cfg(feature = "ssr")]
use leptos::logging::log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (if_show_modal, set_if_show_modal) = create_signal(true);

    provide_context(set_if_show_modal);

    let content = || {
        view! {
            <h4 class="text-xl text-center font-bold leading-6 text-gray-900 mb-2">
                "Menu de Noël à emporter pour le réveillon, à commander dans "<span class="hover:text-red-500"><a href="https://gopal.sumupstore.com/" target="_blank" rel="noreferrer noopener" class="underline hover:decoration-red-500">"notre boutique"</a>" >>"</span>
            </h4>
            <img width=720 height=900 alt="Menu de Noël" src="/assets/img/christmas_menu.webp" />
            <br></br>
            <p class="text-xl font-bold leading-6 text-gray-900 mb-2 text-red-500">
                "Dates d'ouverture du Gopal pour les fêtes"
            </p>
            <ul class="text-lg text-gray-900">
                <li class="mb-2"><span class="font-bold">"Lundi 23 Décembre 16h à 19h et mardi 24 Décembre 11h à 14h :"</span>" ouvert uniquement pour les commandes de menus de Noël à emporter faites sur réservations."</li>
                <li class="mb-2"><span class="font-bold">"Mardi 25 Décembre :"</span>" fermé."</li>
                <li class="mb-2"><span class="font-bold">"Jeudi 26 à samedi 28 Décembre :"</span>" OUVERT aux horaires habituels."</li>
                <li>"Fermeture du 29 Décembre au 6 Janvier pour vacances d'hiver."</li>
            </ul>
        }
    };

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/gopal.css"/>

        // sets the document title
        <Title text="Le Gopal Tours"/>

        <Meta name="description" content="Le Gopal Tours - Restaurant Végétarien & Vegan."/>
        <Meta name="keywords" content="végétarien, vegan, restaurant, gopal, krishna"/>
        <Link rel="apple-touch-icon" href="/logo192.png" />
        <Link rel="manifest" href="/manifest.json" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);

           // Get the current location
           #[cfg(feature = "ssr")]
           let location = use_location();

           // Log the 404 error with the requested path
           #[cfg(feature = "ssr")]
           log!("404 Error: Not Found - Requested Path: {}", location.pathname.get());

            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Header/>
            <Show when=move || { if_show_modal() }>
                <Modal
                    set_if_show_modal
                    title="".to_string()
                    content
                />
            </Show>
            <Routes>
                <Route path="" view=Home/>
                <Route path="menu" view=Menu/>
                <Route path="reservation" view=Reservation/>
                <Route path="contact" view=Contact/>
                <Route path="policy" view=Policy/>
                <Route path="about" view=About/>
                <Route path="gallery" view=Gallery/>
            </Routes>
            <Footer/>
        </Router>
    }
}

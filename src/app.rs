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
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Title};
use leptos_router::components::{Route, Router, Routes};
#[cfg(feature = "ssr")]
use leptos_router::hooks::use_location;
use leptos_router::path;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <link id="leptos" rel="stylesheet" href="/pkg/gopal.css" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (if_show_modal, set_if_show_modal) = signal(false);

    provide_context(set_if_show_modal);

    let content = || {
        view! {
            <ul class="text-lg text-gray-900">
                <li class="mb-2">
                    <span class="font-bold">
                        "Lundi 23 Décembre 16h à 19h et mardi 24 Décembre 11h à 14h :"
                    </span>
                    " ouvert uniquement pour les commandes de menus de Noël à emporter faites sur réservations."
                </li>
                <li class="mb-2">
                    <span class="font-bold">"Mercredi 25 Décembre :"</span>
                    " fermé."
                </li>
                <li class="mb-2">
                    <span class="font-bold">"Jeudi 26 à samedi 28 Décembre :"</span>
                    " OUVERT aux horaires habituels."
                </li>
                <li>"Fermeture du 29 Décembre au 6 Janvier pour vacances d'hiver."</li>
            </ul>
        }
    };

    view! {
        // sets the document title
        <Title text="Le Gopal Tours" />

        <Meta name="description" content="Le Gopal Tours - Restaurant Végétarien & Vegan." />
        <Meta name="keywords" content="végétarien, vegan, restaurant, gopal, krishna" />
        <Link rel="apple-touch-icon" href="/logo192.png" />
        <Link rel="manifest" href="/manifest.json" />

        // content for this welcome page
        <Router>
            <Header />
            <Show when=move || { if_show_modal() }>
                <Modal
                    set_if_show_modal
                    title="Dates d'ouverture du Gopal pour les fêtes".to_string()
                    content
                />
            </Show>
            <Routes fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                #[cfg(feature = "ssr")]
                let location = use_location();
                #[cfg(feature = "ssr")]
                log!("404 Error: Not Found - Requested Path: {}", location.pathname.get());

                // Get the current location

                // Log the 404 error with the requested path

                view! { <ErrorTemplate outside_errors /> }
            }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/menu") view=Menu />
                <Route path=path!("/reservation") view=Reservation />
                <Route path=path!("/contact") view=Contact />
                <Route path=path!("/policy") view=Policy />
                <Route path=path!("/about") view=About />
                <Route path=path!("/gallery") view=Gallery />
            </Routes>
            <Footer />
        </Router>
    }
}

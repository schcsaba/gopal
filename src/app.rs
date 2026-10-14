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
use crate::elements::nav::announcement_enabled;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::static_routes::StaticRoute;
use leptos_router::{path, SsrMode};

pub fn shell() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <MetaTags />
                <link id="leptos" rel="stylesheet" href="/pkg/gopal.css" />
                <script src="/site.js" defer></script>
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
            {announcement_enabled().then(|| view! {
                <Modal
                    title="Dates d'ouverture du Gopal pour les fêtes".to_string()
                    content
                />
            })}
            // Every route is prerendered to `<path>.html` at build time.
            <Routes fallback=NotFound>
                <Route path=path!("/") view=Home ssr=SsrMode::Static(StaticRoute::new()) />
                <Route path=path!("/menu") view=Menu ssr=SsrMode::Static(StaticRoute::new()) />
                <Route path=path!("/reservation") view=Reservation ssr=SsrMode::Static(StaticRoute::new()) />
                <Route path=path!("/contact") view=Contact ssr=SsrMode::Static(StaticRoute::new()) />
                <Route path=path!("/policy") view=Policy ssr=SsrMode::Static(StaticRoute::new()) />
                <Route path=path!("/about") view=About ssr=SsrMode::Static(StaticRoute::new()) />
                <Route path=path!("/gallery") view=Gallery ssr=SsrMode::Static(StaticRoute::new()) />
                // GitHub Pages serves `404.html` for unknown paths.
                <Route path=path!("/404") view=NotFound ssr=SsrMode::Static(StaticRoute::new()) />
            </Routes>
            <Footer />
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    let mut outside_errors = Errors::default();
    outside_errors.insert_with_default_key(AppError::NotFound);
    view! { <ErrorTemplate outside_errors /> }
}

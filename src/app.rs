use crate::elements::footer::Footer;
use crate::elements::header::Header;
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::about::About;
use crate::pages::contact::Contact;
use crate::pages::gallery::Gallery;
use crate::pages::home::Home;
use crate::pages::menu::Menu;
use crate::pages::policy::Policy;
use crate::pages::reservation::Reservation;
use leptos::*;
use leptos_image::provide_image_context;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_image_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/gopal.css"/>

        // sets the document title
        <Title text="Le Gopal Tours"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Header/>
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

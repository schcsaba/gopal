use crate::error_template::{AppError, ErrorTemplate};
use crate::footer::Footer;
use crate::header::Header;
use crate::pages::home_page::HomePage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/gopal.css"/>

        // sets the document title
        <Title text="Leptos + Tailwindcss"/>

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
            <main class="container mx-auto">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}

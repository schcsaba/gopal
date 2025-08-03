use leptos::prelude::*;
use crate::data::menu::{get_formules, get_desserts, get_boissons, MenuItem};

#[component]
pub fn Menu() -> impl IntoView {
    let formules = get_formules();
    let desserts = get_desserts();
    let boissons = get_boissons();

    view! {
        <main class="container mx-auto">
            <section class="mt-20 text-center">
                <h2 class="text-4xl tracking-widest uppercase pb-5">Notre carte</h2>
                <div class="flex justify-center items-center">
                    <img alt="Etagère avec ustensiles de cuisine" src="/assets/img/detourCapture.svg" />
                </div>

                <ul>
                    // Formules section
                    <li class="pt-10 pb-6 text-5xl tracking-widest uppercase">Nos Formules</li>
                    <li class="pb-20 text-2xl">
                        <ul>
                            <For
                                each=move || formules.clone()
                                key=|item| item.name.clone()
                                children=move |item: MenuItem| {
                                    let formatted_name_price = format!("{} {}", item.name, item.format_price());
                                    let is_last = item.name == "Spéciale du chef";
                                    
                                    view! {
                                        <li class="py-4 font-sans font-extrabold">
                                            {formatted_name_price}
                                        </li>
                                        <li class={if is_last { "" } else { "mb-8" }}>
                                            {if item.name == "Plat du jour" {
                                                view! {
                                                    "riz parfumé + curry " <span class="font-extrabold">"ou"</span> " boulettes de légumes " <span class="font-extrabold">"ou"</span> " soupe (au choix) + galette papadam"
                                                }.into_any()
                                            } else {
                                                view! { {item.description} }.into_any()
                                            }}
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </li>
                    
                    // Desserts section
                    <li class="pt-10 pb-6 text-5xl tracking-widest uppercase">Nos Desserts</li>
                    <li class="pb-20 text-2xl">
                        <ul>
                            <For
                                each=move || desserts.clone()
                                key=|item| item.name.clone()
                                children=move |item: MenuItem| {
                                    let formatted_text = if item.description.is_empty() {
                                        format!("{} {}", item.name, item.format_price())
                                    } else {
                                        format!("{} ({}) {}", item.name, item.description, item.format_price())
                                    };
                                    
                                    view! {
                                        <li class="py-4 mb-4 font-sans font-extrabold">
                                            {formatted_text}
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </li>
                    
                    // Boissons section
                    <li class="pt-10 pb-6 text-5xl tracking-widest uppercase">Nos Boissons</li>
                    <li class="pb-20 text-2xl">
                        <ul>
                            <For
                                each=move || boissons.clone()
                                key=|item| item.name.clone()
                                children=move |item: MenuItem| {
                                    let formatted_text = if item.description.is_empty() {
                                        format!("{} {}", item.name, item.format_price())
                                    } else {
                                        format!("{} ({}) {}", item.name, item.description, item.format_price())
                                    };
                                    
                                    view! {
                                        <li class="py-4 mb-4 font-sans font-extrabold">
                                            {formatted_text}
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </li>
                </ul>
            </section>
        </main>
    }
}

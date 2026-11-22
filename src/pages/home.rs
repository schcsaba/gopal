use crate::data::contact::get_contact_info;
use crate::data::menu::get_formules;
use crate::elements::video::Video;
use crate::pages::menu::{MenuLine, Swash};
use leptos::prelude::*;

/// The Google map of the restaurant, used here and on the contact page. It
/// fills its parent, which must be positioned and sized.
#[component]
pub fn MapEmbed() -> impl IntoView {
    view! {
        <div class="absolute inset-0">
            <iframe
                src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2700.13273053803!2d0.6773934768938756!3d47.4093521711725!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47fcd5dbe973d05f%3A0x4623a58782228f72!2sGopal!5e0!3m2!1sen!2sfr!4v1723976237764!5m2!1sen!2sfr"
                class="block size-full"
                style="border:0;"
                allowfullscreen=""
                loading="lazy"
                referrerpolicy="no-referrer-when-downgrade"
                title="Google Map"
            ></iframe>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let contact = get_contact_info();
    let address_formatted = contact.address.replace(", ", "<br />");
    let phone_formatted = contact.format_phone_for_display();
    let phone_href = format!("tel:{}", contact.phone);
    let mut hours = contact.hours.clone();
    // Single-service days first, then the day with an evening service on
    // its own line.
    hours.sort_by_key(|day| day.slots.len());
    let formules = get_formules().into_iter().take(3).collect::<Vec<_>>();

    view! {
        <main id="contenu">
            // Hero
            <section class="bg-papier">
                <div class="wrap grid items-center gap-12 py-8 lg:min-h-[48.75rem] lg:grid-cols-[1.15fr_1fr] lg:gap-16 lg:py-16">
                    <div class="flex flex-col gap-6 lg:gap-7">
                        <p class="eyebrow text-sm text-piment lg:text-base">"Créé à Tours en Novembre 2019"</p>
                        <h1 class="font-serif text-[3.25rem] leading-[0.98] font-normal tracking-[-0.02em] sm:text-7xl lg:text-[5.75rem]">
                            "Tous nos plats sont "
                            <em class="text-piment">"faits maison."</em>
                        </h1>
                        <Swash class="w-64 lg:w-[21.25rem]" />
                        <div class="flex flex-wrap gap-4">
                            <a class="btn btn-encre" href="/menu">"Aller au menu" <span aria-hidden="true">"→"</span></a>
                            <a class="btn btn-contour hidden sm:inline-flex" href="/reservation">"Réservation"</a>
                        </div>
                    </div>
                    <div class="order-first lg:order-none">
                        <div class="frame frame-br">
                            <Video />
                            <div
                                aria-hidden="true"
                                class="absolute -top-5 -right-2 flex size-24 rotate-[8deg] items-center justify-center rounded-full bg-curcuma text-center font-serif text-[0.9375rem] leading-[1.05] text-encre italic lg:-top-7 lg:-right-3.5 lg:size-33 lg:text-[1.375rem]"
                            >
                                "100%"<br />"végétarienne"<br />"et végane"
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // N° 01 · Découvrez notre menu
            <section class="border-t border-ligne bg-papier">
                <div class="wrap flex flex-wrap-reverse items-center gap-12 py-16 lg:min-h-[47.5rem] lg:gap-18 lg:py-20">
                    <div class="frame frame-tl min-w-0 flex-[1_1_32rem]">
                        <img
                            class="block h-72 w-full object-cover sm:h-96 lg:h-[32.5rem]"
                            width="1080"
                            height="894"
                            loading="lazy"
                            alt="Découvrez notre menu"
                            src="/assets/img/menu.webp"
                        />
                    </div>
                    <div class="flex min-w-0 flex-[1_1_26rem] flex-col gap-6">
                        <p class="font-serif text-2xl text-piment italic">"N° 01"</p>
                        <h2 class="font-serif text-5xl leading-[1.02] font-medium tracking-[-0.01em] lg:text-6xl">"Découvrez notre menu"</h2>
                        <p class="text-xl leading-relaxed text-mousse lg:text-[1.375rem]">
                            "Au Gopal venez découvrir notre cuisine végétarienne et végane Internationale,
                            aussi bien inspirée des saveurs indiennes que des bons petits plats de chez nous."
                        </p>
                        <ul class="flex flex-col gap-2.5 py-2 font-serif text-[1.4375rem]">
                            {formules.into_iter().map(|item| view! {
                                <li><MenuLine name=item.name.clone() price=item.format_price() /></li>
                            }).collect_view()}
                        </ul>
                        <a class="btn btn-encre self-start" href="/menu">"Aller au menu" <span aria-hidden="true">"→"</span></a>
                    </div>
                </div>
            </section>

            // N° 02–05
            <section class="bg-sable">
                <div class="wrap grid grid-cols-1 gap-x-8 gap-y-14 py-20 sm:grid-cols-2 lg:py-24 xl:grid-cols-4">
                    <Feature number="N° 02" title="À propos de nous" href=Some("/about") image="/assets/img/about.webp" size=2048>
                        "Créé à Tours en Novembre 2019, le Gopal vous propose de venir déguster une cuisine 100% végétarienne
                        et végane dans un cadre calme et serein."
                    </Feature>
                    <Feature number="N° 03" title="Spécialités" href=Some("/menu") image="/assets/img/specialities.webp" size=773>
                        "Découvrez nos spécialités végétariennes et véganes, alliant cuisine traditionnelle et inspirations
                        internationales, avec des plats savoureux comme notre curry parfumé, nos lasagnes royales, ou encore
                        nos desserts gourmands."
                    </Feature>
                    <Feature number="N° 04" title="Galerie" href=Some("/gallery") image="/assets/img/gallery.webp" size=987>
                        "Explorez notre galerie pour un aperçu en images de nos délicieuses préparations culinaires
                        et de l’atmosphère chaleureuse de notre restaurant."
                    </Feature>
                    <Feature number="N° 05" title="Evénements" href=None image="/assets/img/events.webp" size=924>
                        "Participez à nos soirées kirtan (dans la salle de yoga du Gopal) chaque vendredi de 19h00 à 20h30, et laissez-vous emporter par une
                        méditation musicale apaisante dans une ambiance conviviale."
                    </Feature>
                </div>
            </section>

            // N° 06 · Que des bonnes vibrations
            <section class="bg-encre text-papier">
                <div class="wrap flex flex-wrap items-center gap-12 py-20 lg:min-h-[51.25rem] lg:gap-18 lg:py-24">
                    <div class="flex min-w-0 flex-[1_1_26rem] flex-col gap-7">
                        <p class="font-serif text-2xl text-curcuma italic">"N° 06"</p>
                        <h2 class="font-serif text-[3.25rem] leading-[0.98] font-normal tracking-[-0.02em] sm:text-7xl lg:text-[5.5rem]">
                            "Que des " <em class="text-curcuma-clair">"bonnes"</em> " vibrations"
                        </h2>
                        <p class="max-w-[32.5rem] text-xl leading-relaxed text-brume lg:text-[1.375rem]">
                            "Venez partager des moments de convivialité dans une ambiance joyeuse et positive, où bonne humeur et bien-être sont toujours au rendez-vous."
                        </p>
                    </div>
                    <div class="frame frame-bl min-w-0 flex-[1_1_32rem] [--frame-color:var(--color-curcuma)]">
                        <img
                            class="block h-72 w-full object-cover sm:h-96 lg:h-[33.75rem]"
                            width="2048"
                            height="1536"
                            loading="lazy"
                            alt="Que des bonnes vibrations"
                            src="/assets/img/good_vibes.webp"
                        />
                    </div>
                </div>
            </section>

            // Rendez-nous visite
            <section class="bg-papier">
                <div class="wrap flex flex-wrap items-stretch gap-14 py-20 lg:py-24">
                    <div class="flex min-w-0 flex-[1_1_28rem] flex-col gap-2.5">
                        <h2 class="font-serif text-4xl leading-[1.05] font-medium lg:text-5xl">"Rendez-nous visite :"</h2>
                        <p class="pt-3.5 text-lg font-bold tracking-[0.16em] uppercase">"Le Gopal"</p>
                        <p class="text-xl leading-normal lg:text-[1.375rem]" inner_html=address_formatted></p>
                        <h2 class="mt-7 font-serif text-4xl leading-[1.05] font-medium lg:text-5xl">"Appelez-nous :"</h2>
                        <p class="text-2xl lg:text-[1.625rem]">
                            <a class="lien" href=phone_href>{phone_formatted}</a>
                        </p>
                        <ul class="mt-3 grid grid-cols-1 gap-x-6 gap-y-1.5 border-y border-ligne py-6 text-lg sm:grid-cols-2">
                            {hours.into_iter().map(|day| {
                                let evening = day.slots.len() > 1;
                                let times = day.slots.iter().map(|slot| slot.format()).collect::<Vec<_>>().join(" et ");
                                view! {
                                    <li class=("sm:col-span-2", evening)>
                                        <span class="font-serif text-xl font-medium" class=("text-piment", evening)>{day.day.to_french()}</span>
                                        " · " {times}
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                        <a class="btn btn-piment mt-4 self-start" href="/reservation">"Réservation"</a>
                    </div>
                    <div class="relative min-h-[22rem] min-w-0 flex-[1_1_32rem] border-[1.5px] border-encre bg-kraft lg:min-h-[32.5rem]">
                        <MapEmbed />
                    </div>
                </div>
            </section>

            // Google reviews
            <section class="border-t border-ligne bg-papier">
                <div class="wrap py-16">
                    <iframe class="h-screen w-full" src="https://2bc10f5dd2994533866aac1fe6e8e6fb.elf.site" title="Google Reviews"></iframe>
                </div>
            </section>
        </main>
    }
}

/// One of the four numbered teasers on the home page.
#[component]
fn Feature(
    number: &'static str,
    title: &'static str,
    href: Option<&'static str>,
    image: &'static str,
    /// The square source image's side, in pixels.
    size: u32,
    children: Children,
) -> impl IntoView {
    let img = view! {
        <img
            class="block aspect-[4/5] w-full object-cover"
            class=("grayscale", href.is_none())
            width=size
            height=size
            loading="lazy"
            alt=title
            src=image
        />
    };
    view! {
        <article class="flex flex-col gap-3.5">
            {match href {
                Some(href) => view! { <a href=href tabindex="-1" aria-hidden="true">{img}</a> }.into_any(),
                None => img.into_any(),
            }}
            <p class="pt-2 font-serif text-xl text-piment-fonce italic">{number}</p>
            <h2 class="font-serif text-[2.125rem] leading-[1.1] font-medium">
                {match href {
                    Some(href) => view! { <a class="hover:text-piment" href=href>{title}</a> }.into_any(),
                    None => title.into_any(),
                }}
            </h2>
            <p class="text-lg leading-relaxed text-mousse lg:text-[1.1875rem]">{children()}</p>
        </article>
    }
}

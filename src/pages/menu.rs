use crate::data::menu::{get_boissons, get_desserts, get_formules, MenuItem};
use leptos::prelude::*;

#[component]
pub fn Menu() -> impl IntoView {
    let chip = "btn btn-contour min-h-12 shrink-0 px-5 text-base tracking-[0.1em] lg:min-h-12 lg:px-5.5 lg:text-[1.0625rem]";

    view! {
        <main id="contenu">
            <section class="bg-papier">
                <div class="wrap flex flex-col gap-5 py-10 lg:min-h-[26.25rem] lg:items-center lg:justify-center lg:py-18 lg:text-center">
                    <h1 class="font-serif text-6xl leading-[0.95] font-normal tracking-[-0.03em] lg:text-[8rem]">
                        "Notre " <em class="text-piment">"carte"</em>
                    </h1>
                    <Swash class="w-56 lg:w-[18.75rem]" />
                    <nav aria-label="Sections de la carte" class="-mx-5 overflow-x-auto px-5 pb-1 [scrollbar-width:none] lg:mx-0 lg:overflow-visible lg:px-0">
                        <ul class="flex gap-3 pt-3 lg:justify-center">
                            <li><a class=chip href="#formules">"Nos Formules"</a></li>
                            <li><a class=chip href="#desserts">"Nos Desserts"</a></li>
                            <li><a class=chip href="#boissons">"Nos Boissons"</a></li>
                        </ul>
                    </nav>
                </div>
            </section>

            <section id="formules" aria-labelledby="formules-titre" class="bg-papier">
                <div class="wrap pt-2 pb-20 lg:pt-6 lg:pb-24">
                    <div class="ardoise on-ardoise flex flex-col gap-10 px-6 py-10 sm:px-12 sm:py-14 lg:gap-12 lg:px-18 lg:pt-18 lg:pb-20">
                        <div class="flex flex-wrap items-end justify-between gap-x-6 gap-y-3">
                            <h2 id="formules-titre" class="font-serif text-4xl leading-none font-normal italic sm:text-5xl lg:text-7xl">"Nos Formules"</h2>
                            <p class="eyebrow text-[0.9375rem] tracking-[0.2em] text-curcuma-clair lg:text-[1.0625rem]">"Tous nos plats sont faits maison."</p>
                        </div>
                        <ul class="grid grid-cols-1 gap-x-18 gap-y-9 lg:grid-cols-2 lg:gap-y-14">
                            {get_formules().into_iter().map(|item| view! {
                                <li class="flex flex-col gap-2 lg:gap-3.5">
                                    <MenuLine
                                        name=item.name.clone()
                                        price=item.format_price()
                                        class="font-serif text-[1.4375rem] leading-[1.15] sm:text-3xl lg:text-[2.375rem]"
                                        price_class="text-curcuma-clair"
                                    />
                                    <p class="text-lg leading-relaxed text-craie-douce lg:text-[1.3125rem]">
                                        <Description item=item.clone() emphasis="font-bold text-craie" />
                                    </p>
                                </li>
                            }).collect_view()}
                        </ul>
                        <div class="grid grid-cols-2 gap-4 pt-2 lg:grid-cols-4 lg:gap-5">
                            <Polaroid src="/assets/img/gallery/118652653_327236958690345_4313966520127569303_n.webp" alt="Menu « Gopal »" width=2016 height=1512 tilt="-rotate-[1.5deg]" />
                            <Polaroid src="/assets/img/gallery/spinach_curry_with_curd.webp" alt="Curry" width=1080 height=793 tilt="rotate-1" />
                            <Polaroid src="/assets/img/specialities.webp" alt="Spéciale du chef" width=773 height=773 tilt="-rotate-[0.5deg]" />
                            <Polaroid src="/assets/img/gallery/277467988_718498789564158_4364983103444112164_n.webp" alt="Samosa aux légumes" width=2016 height=1512 tilt="rotate-[1.5deg]" />
                        </div>
                    </div>
                </div>
            </section>

            <section class="border-t border-ligne bg-papier">
                <div class="wrap flex flex-wrap gap-x-20 gap-y-18 py-20 lg:py-24">
                    <Course
                        id="desserts"
                        title="Desserts"
                        items=get_desserts()
                        image="/assets/img/gallery/cake.webp"
                        width=1718
                        height=1374
                    />
                    <Course
                        id="boissons"
                        title="Boissons"
                        items=get_boissons()
                        image="/assets/img/gallery/178992870_499100254837347_8234335642050262977_n.webp"
                        width=1018
                        height=1272
                    />
                </div>
            </section>
        </main>
    }
}

/// A dish and its price, joined by a dotted line.
#[component]
pub fn MenuLine(
    name: String,
    price: String,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] price_class: &'static str,
) -> impl IntoView {
    view! {
        <p class=format!("leader {class}")>
            <span>{name}</span>
            <span class=price_class>{price}</span>
        </p>
    }
}

/// The hand-drawn turmeric stroke under the big headlines.
#[component]
pub fn Swash(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <svg class=format!("h-4 {class}") viewBox="0 0 340 16" preserveAspectRatio="none" aria-hidden="true">
            <path d="M3 11 C 90 3, 230 2, 337 9" stroke="#D9A441" stroke-width="4" fill="none" stroke-linecap="round" vector-effect="non-scaling-stroke" />
        </svg>
    }
}

/// A dish's description, with the alternatives of the "Plat du jour" set off.
#[component]
fn Description(item: MenuItem, emphasis: &'static str) -> impl IntoView {
    if item.name == "Plat du jour" {
        view! {
            "riz parfumé + curry " <b class=emphasis>"ou"</b> " boulettes de légumes " <b class=emphasis>"ou"</b> " soupe (au choix) + galette papadam"
        }
        .into_any()
    } else {
        item.description.into_any()
    }
}

/// A photo pinned to the chalkboard.
#[component]
fn Polaroid(
    src: &'static str,
    alt: &'static str,
    width: u32,
    height: u32,
    tilt: &'static str,
) -> impl IntoView {
    view! {
        <img
            class=format!("block h-36 w-full border-[6px] border-craie object-cover sm:h-48 lg:h-55 {tilt}")
            loading="lazy"
            width=width
            height=height
            src=src
            alt=alt
        />
    }
}

/// Desserts or drinks: a framed photo, the heading and the list.
#[component]
fn Course(
    id: &'static str,
    title: &'static str,
    items: Vec<MenuItem>,
    image: &'static str,
    width: u32,
    height: u32,
) -> impl IntoView {
    let heading_id = format!("{id}-titre");
    view! {
        <div id=id class="flex min-w-0 flex-[1_1_30rem] flex-col gap-5">
            <div class="frame frame-br mb-5">
                <img
                    class="block h-56 w-full object-cover sm:h-75"
                    loading="lazy"
                    width=width
                    height=height
                    src=image
                    alt=format!("Nos {title}")
                />
            </div>
            <h2 id=heading_id.clone() class="mb-2 font-serif text-5xl leading-none font-normal lg:text-6xl">
                "Nos " <em class="text-piment">{title}</em>
            </h2>
            <ul aria-labelledby=heading_id class="flex flex-col gap-5">
                {items.into_iter().map(|item| {
                    let has_description = !item.description.is_empty();
                    view! {
                        <li class="flex flex-col gap-1">
                            <MenuLine
                                name=item.name.clone()
                                price=item.format_price()
                                class="font-serif text-2xl leading-snug lg:text-[1.625rem]"
                            />
                            {has_description.then(|| view! {
                                <p class="text-lg text-lichen italic">{item.description.clone()}</p>
                            })}
                        </li>
                    }
                }).collect_view()}
            </ul>
        </div>
    }
}

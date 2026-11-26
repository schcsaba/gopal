use crate::data::noel::{
    get_buches, get_noel_courses, Buche, Dish, BUCHES_URL, BUCHE_PRICE, BUCHE_SIZE,
    ORDER_DEADLINE, PHONE, PHONE_DISPLAY, PICKUP, PRICE, SHOP_URL,
};
use crate::pages::menu::{MenuLine, Swash};
use leptos::prelude::*;

/// Christmas to take away: the menu and the bûches, when to order and collect
/// them, and the way to the online shop.
#[component]
pub fn Noel() -> impl IntoView {
    let phone_href = format!("tel:{PHONE}");
    let fact = "flex flex-col gap-0.5 border-t border-ligne py-3 sm:flex-row sm:items-baseline sm:gap-6";
    let fact_label = "eyebrow shrink-0 text-[0.9375rem] text-lichen sm:w-44";
    let price = "font-serif text-3xl text-piment";
    let see = "lien ml-3 text-lg";

    view! {
        <main id="contenu">
            <section class="bg-papier">
                <div class="wrap grid items-center gap-14 py-10 lg:grid-cols-[1.25fr_1fr] lg:gap-18 lg:py-14">
                    <div class="flex flex-col gap-5">
                        <p class="eyebrow text-sm text-piment lg:text-base">"À emporter · Noël 2026"</p>
                        <h1 class="font-serif text-6xl leading-[0.95] font-normal tracking-[-0.03em] sm:text-7xl lg:text-[6rem]">
                            "Noël " <em class="text-piment">"à emporter"</em>
                        </h1>
                        <Swash class="w-56 lg:w-[18.75rem]" />
                        <p class="max-w-[34rem] font-serif text-2xl leading-snug lg:text-[1.875rem]">
                            "Un repas festif et des bûches gourmandes, faits maison avec amour !"
                        </p>
                        <dl class="mt-2 border-b border-ligne text-xl lg:text-[1.3125rem]">
                            <div class=fact>
                                <dt class=fact_label>"Menu de Noël"</dt>
                                <dd>
                                    <span class=price>{PRICE}</span>
                                    <a class=see href="#menu">"Voir le menu"</a>
                                </dd>
                            </div>
                            <div class=fact>
                                <dt class=fact_label>"Bûche de Noël"</dt>
                                <dd>
                                    <span class=price>{BUCHE_PRICE}</span>
                                    <a class=see href="#buches">"Voir les bûches"</a>
                                </dd>
                            </div>
                            <div class=fact>
                                <dt class=fact_label>"Commandes"</dt>
                                <dd>"jusqu’au " <strong>{ORDER_DEADLINE}</strong></dd>
                            </div>
                            <div class=fact>
                                <dt class=fact_label>"Retrait"</dt>
                                <dd>"au restaurant, " <strong>{PICKUP}</strong></dd>
                            </div>
                        </dl>
                        <div class="flex flex-wrap gap-4 pt-2">
                            <a class="btn btn-piment" href=SHOP_URL>
                                "Commander en ligne" <span aria-hidden="true">"→"</span>
                            </a>
                            <a class="btn btn-contour tracking-wide" href=phone_href>
                                <span class="sr-only">"Appeler le "</span>{PHONE_DISPLAY}
                            </a>
                        </div>
                    </div>
                    <figure class="mx-auto w-full max-w-[32rem]">
                        <div class="grid grid-cols-2 items-start gap-4 sm:gap-6">
                            <Poster
                                viewer="affiche-menu"
                                src="/assets/img/menu-noel.webp"
                                width=1078
                                height=1600
                                alt="L’affiche du menu de Noël à emporter du Gopal"
                                tilt="-rotate-2"
                            />
                            <Poster
                                viewer="affiche-buches"
                                src="/assets/img/buches-noel.webp"
                                width=1131
                                height=1600
                                alt="L’affiche des bûches de Noël à emporter du Gopal"
                                tilt="mt-8 rotate-2"
                            />
                        </div>
                        <figcaption class="pt-5 text-center text-base text-lichen">
                            "Cliquez sur une affiche pour l’agrandir"
                        </figcaption>
                    </figure>
                </div>
            </section>

            <section id="menu" aria-labelledby="noel-menu-titre" class="bg-papier">
                <div class="wrap pt-2 pb-20 lg:pb-24">
                    <div class="ardoise on-ardoise flex flex-col gap-10 px-6 py-10 sm:px-12 sm:py-14 lg:gap-14 lg:px-18 lg:pt-18 lg:pb-20">
                        <div class="flex flex-wrap items-end justify-between gap-x-6 gap-y-3">
                            <h2 id="noel-menu-titre" class="font-serif text-4xl leading-none font-normal italic sm:text-5xl lg:text-7xl">
                                "Au menu"
                            </h2>
                            <p class="eyebrow text-[0.9375rem] tracking-[0.2em] text-curcuma-clair lg:text-[1.0625rem]">
                                "Un choix à chaque étape"
                            </p>
                        </div>
                        <div class="grid grid-cols-1 gap-x-18 gap-y-12 lg:grid-cols-2 lg:gap-y-16">
                            {get_noel_courses().into_iter().map(|course| {
                                let [first, second] = course.choices;
                                view! {
                                    <div class="flex flex-col gap-4">
                                        <h3 class="border-b border-craie/25 pb-3 font-serif text-3xl leading-tight text-curcuma-clair italic lg:text-[2.375rem]">
                                            {course.title}
                                        </h3>
                                        <Choice dish=first />
                                        <p aria-hidden="true" class="flex items-center gap-4 font-serif text-xl text-craie-douce italic">
                                            <span class="h-px w-8 bg-craie/30"></span>"ou"<span class="h-px w-8 bg-craie/30"></span>
                                        </p>
                                        <p class="sr-only">"ou"</p>
                                        <Choice dish=second />
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                        <div class="flex flex-col gap-6 border-t border-craie/25 pt-8 lg:flex-row lg:items-end lg:justify-between lg:gap-12 lg:pt-10">
                            <div class="flex min-w-0 flex-1 flex-col gap-3">
                                <MenuLine
                                    name="Le menu complet, à emporter".to_string()
                                    price=PRICE.to_string()
                                    class="font-serif text-2xl leading-tight sm:text-3xl lg:text-[2.375rem]"
                                    price_class="text-curcuma-clair"
                                />
                                <p class="text-lg text-craie-douce">
                                    <span aria-hidden="true"><Vegan /> " végane · "</span>
                                    "Tous nos plats sont faits maison."
                                </p>
                            </div>
                            <a class="btn btn-piment self-start lg:self-auto" href=SHOP_URL>
                                "Commander en ligne" <span aria-hidden="true">"→"</span>
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            <section id="buches" aria-labelledby="buches-titre" class="border-t border-ligne bg-sable">
                <div class="wrap flex flex-col gap-10 py-20 lg:gap-12 lg:py-24">
                    <div class="flex flex-wrap items-end justify-between gap-x-10 gap-y-4">
                        <h2 id="buches-titre" class="font-serif text-5xl leading-none font-normal lg:text-7xl">
                            "Nos " <em class="text-piment">"bûches"</em> " de Noël"
                        </h2>
                        <p class="text-xl text-mousse lg:text-[1.375rem]">
                            "Taille " {BUCHE_SIZE} " · à commander avant le " {ORDER_DEADLINE}
                        </p>
                    </div>
                    <p class="-mt-4 flex items-center gap-3 text-lg text-mousse lg:-mt-6">
                        <span aria-hidden="true"><Vegan /></span>
                        "Nos deux bûches sont véganes et existent aussi en version sans gluten."
                    </p>
                    <ul class="grid grid-cols-1 gap-8 lg:grid-cols-2">
                        {get_buches().into_iter().map(|buche| view! { <li><BucheCard buche /></li> }).collect_view()}
                    </ul>
                    <div class="flex flex-wrap items-center gap-x-8 gap-y-4">
                        <a class="btn btn-piment" href=BUCHES_URL>
                            "Commander une bûche" <span aria-hidden="true">"→"</span>
                        </a>
                        <a
                            class="lien text-lg"
                            href="/assets/img/buches-noel.webp"
                            data-lightbox-open
                            aria-controls="affiche-buches"
                            aria-haspopup="dialog"
                        >
                            "Voir l’affiche des bûches"
                        </a>
                    </div>
                </div>
            </section>

            <PosterViewer
                id="affiche-menu"
                label="L’affiche du menu de Noël"
                src="/assets/img/menu-noel.webp"
                width=1078
                height=1600
                alt="L’affiche du menu de Noël à emporter du Gopal"
            />
            <PosterViewer
                id="affiche-buches"
                label="L’affiche des bûches de Noël"
                src="/assets/img/buches-noel.webp"
                width=1131
                height=1600
                alt="L’affiche des bûches de Noël à emporter du Gopal"
            />
        </main>
    }
}

/// One of the two bûches, as a card with its price.
#[component]
fn BucheCard(buche: Buche) -> impl IntoView {
    view! {
        <article class="flex h-full flex-col gap-4 border-[1.5px] border-encre bg-papier px-5 py-7 sm:px-9 sm:py-9">
            <p class="leader font-serif text-[1.375rem] leading-tight sm:text-3xl lg:text-[2.375rem]">
                <span>{buche.name} {buche.is_vegan.then(|| view! { " " <Vegan /> })}</span>
                <span class="text-piment">{BUCHE_PRICE}</span>
            </p>
            <p class="text-xl leading-relaxed text-mousse lg:text-[1.3125rem]">{buche.description}</p>
        </article>
    }
}

/// A poster pinned at a slight angle; it opens full size in its viewer.
/// Without JavaScript the link shows the image itself.
#[component]
fn Poster(
    viewer: &'static str,
    src: &'static str,
    width: u32,
    height: u32,
    alt: &'static str,
    tilt: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=src
            data-lightbox-open
            aria-controls=viewer
            aria-haspopup="dialog"
            class=format!("block cursor-zoom-in border-[6px] border-creme shadow-[0_1.25rem_2.5rem_rgb(30_51_40/0.2)] transition-transform duration-300 hover:rotate-0 {tilt}")
        >
            <img class="block h-auto w-full" width=width height=height src=src alt=alt />
        </a>
    }
}

/// A poster at full size over the page. `public/site.js` opens it from any
/// `data-lightbox-open` link whose `aria-controls` names it, and closes it on
/// the button, Escape, or a click beside the poster.
#[component]
fn PosterViewer(
    id: &'static str,
    label: &'static str,
    src: &'static str,
    width: u32,
    height: u32,
    alt: &'static str,
) -> impl IntoView {
    view! {
        <dialog
            id=id
            data-lightbox
            aria-label=label
            class="fixed inset-0 m-0 h-dvh max-h-none w-screen max-w-none flex-col overflow-y-auto overscroll-contain bg-encre/95 p-0 text-papier open:flex backdrop:bg-transparent"
        >
            <div class="sticky top-0 z-10 flex justify-end p-3 sm:p-4">
                <button
                    data-lightbox-close
                    type="button"
                    class="btn min-h-12 cursor-pointer gap-2 bg-papier px-5 text-encre shadow-lg hover:bg-curcuma-clair lg:min-h-13"
                >
                    "Fermer" <span aria-hidden="true" class="text-2xl leading-none">"×"</span>
                </button>
            </div>
            <div class="flex flex-1 items-start justify-center px-3 pb-6 sm:px-6 lg:-mt-10 lg:items-center">
                <img
                    class="block h-auto w-full max-w-[1131px] lg:max-h-[calc(100dvh-3rem)] lg:w-auto"
                    width=width
                    height=height
                    loading="lazy"
                    src=src
                    alt=alt
                />
            </div>
        </dialog>
    }
}

/// One of the two dishes offered for a course.
#[component]
fn Choice(dish: Dish) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <p class="font-serif text-2xl leading-snug lg:text-[1.75rem]">
                {dish.name} {dish.is_vegan.then(|| view! { " " <Vegan /> })}
            </p>
            <p class="text-lg leading-relaxed text-craie-douce lg:text-[1.1875rem]">{dish.description}</p>
        </div>
    }
}

/// The "V" mark of a vegan dish, as on the poster.
#[component]
fn Vegan() -> impl IntoView {
    view! {
        <span class="inline-flex size-7 translate-y-[-0.15em] items-center justify-center rounded-full bg-curcuma align-middle font-sans text-sm font-bold text-encre not-italic" title="Végane">
            <span aria-hidden="true">"V"</span>
            <span class="sr-only">"(végane)"</span>
        </span>
    }
}

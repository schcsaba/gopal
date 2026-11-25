use crate::data::noel::{
    get_noel_courses, Dish, ORDER_DEADLINE, PHONE, PHONE_DISPLAY, PICKUP, PRICE, SHOP_URL,
};
use crate::pages::menu::{MenuLine, Swash};
use leptos::prelude::*;

/// The Christmas take-away menu: what's in it, when to order and collect it,
/// and the way to the online shop.
#[component]
pub fn Noel() -> impl IntoView {
    let phone_href = format!("tel:{PHONE}");
    let fact = "flex flex-col gap-0.5 border-t border-ligne py-3 sm:flex-row sm:items-baseline sm:gap-6";
    let fact_label = "eyebrow shrink-0 text-[0.9375rem] text-lichen sm:w-36";

    view! {
        <main id="contenu">
            <section class="bg-papier">
                <div class="wrap grid items-center gap-14 py-10 lg:grid-cols-[1.35fr_1fr] lg:gap-18 lg:py-14">
                    <div class="flex flex-col gap-5">
                        <p class="eyebrow text-sm text-piment lg:text-base">"À emporter · Noël 2026"</p>
                        <h1 class="font-serif text-6xl leading-[0.95] font-normal tracking-[-0.03em] sm:text-7xl lg:text-[6rem]">
                            "Menu de " <em class="text-piment">"Noël"</em>
                        </h1>
                        <Swash class="w-56 lg:w-[18.75rem]" />
                        <p class="max-w-[34rem] font-serif text-2xl leading-snug lg:text-[1.875rem]">
                            "Un repas festif, gourmand et fait maison avec amour !"
                        </p>
                        <dl class="mt-2 border-b border-ligne text-xl lg:text-[1.3125rem]">
                            <div class=fact>
                                <dt class=fact_label>"Prix"</dt>
                                <dd class="font-serif text-3xl text-piment">{PRICE}</dd>
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
                    <figure class="mx-auto w-full max-w-[28rem] lg:max-w-[26rem]">
                        // Opens the poster in the viewer below; without
                        // JavaScript the link shows the image itself.
                        <a
                            href="/assets/img/menu-noel.webp"
                            data-lightbox-open
                            aria-controls="affiche"
                            aria-haspopup="dialog"
                            class="frame frame-br block cursor-zoom-in [--frame-fill:var(--color-kraft)]"
                        >
                            <img
                                class="block h-auto w-full"
                                width="1078"
                                height="1600"
                                src="/assets/img/menu-noel.webp"
                                alt="L’affiche du menu de Noël à emporter du Gopal"
                            />
                        </a>
                        <figcaption class="pt-3 text-center text-base text-lichen">
                            "Cliquez pour agrandir l’affiche"
                        </figcaption>
                    </figure>
                </div>
            </section>

            <section aria-labelledby="noel-menu-titre" class="bg-papier">
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

            <PosterViewer />
        </main>
    }
}

/// The poster at full size over the page. `public/site.js` opens it from the
/// `data-lightbox-open` link and closes it on the button, Escape, or a click
/// beside the poster.
#[component]
fn PosterViewer() -> impl IntoView {
    view! {
        <dialog
            id="affiche"
            data-lightbox
            aria-label="L’affiche du menu de Noël"
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
                    class="block h-auto w-full max-w-[1078px] lg:max-h-[calc(100dvh-3rem)] lg:w-auto"
                    width="1078"
                    height="1600"
                    loading="lazy"
                    src="/assets/img/menu-noel.webp"
                    alt="L’affiche du menu de Noël à emporter du Gopal"
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

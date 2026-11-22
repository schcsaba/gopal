use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Images above this index are lazy-loaded by the browser.
const EAGER_IMAGES: usize = 8;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: usize,
    pub filename: String,
    pub aspect_ratio: f32,
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.filename == other.filename
            && (self.aspect_ratio - other.aspect_ratio).abs() < f32::EPSILON
    }
}

/// Lists the `.webp` files in the gallery folder, sorted by filename.
/// Read once at build time.
pub fn get_images() -> Vec<Image> {
    use std::fs;

    let gallery_path =
        std::env::var("GALLERY_PATH").unwrap_or_else(|_| "public/assets/img/gallery".to_string());

    let mut filenames: Vec<String> = fs::read_dir(gallery_path)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
        .filter(|filename| filename.ends_with(".webp"))
        .collect();
    filenames.sort();

    filenames
        .into_iter()
        .enumerate()
        .map(|(id, filename)| Image {
            id,
            filename,
            aspect_ratio: 1.5,
        })
        .collect()
}

// Click-to-expand is wired up by `public/site.js` through `data-gallery-item`.
#[component]
pub fn Gallery() -> impl IntoView {
    view! {
        <main id="contenu" class="wrap py-12 lg:py-18">
            <h1 class="pb-10 font-serif text-5xl leading-none font-normal tracking-[-0.02em] sm:text-6xl lg:pb-14 lg:text-8xl">"Galerie"</h1>
            <div class="grid grid-cols-2 gap-3 md:grid-cols-3 md:gap-4 lg:grid-cols-4 xl:grid-cols-5">
                {get_images().into_iter().map(|image| {
                    view! {
                        <div
                            data-gallery-item
                            class="relative cursor-pointer overflow-hidden bg-kraft transition-all duration-700 ease-in-out hover:scale-105"
                        >
                            <img
                                src=format!("/assets/img/gallery/{}", image.filename)
                                alt=format!("Image {}", image.id)
                                loading=if image.id < EAGER_IMAGES { "eager" } else { "lazy" }
                                decoding="async"
                                class="w-full h-auto object-cover"
                                style=format!("aspect-ratio: {}", image.aspect_ratio)
                            />
                        </div>
                    }
                }).collect_view()}
            </div>
        </main>
    }
}

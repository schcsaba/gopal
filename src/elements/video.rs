use leptos::prelude::*;

/// The restaurant clip, muted and looping, with a pause button. `public/site.js`
/// wires the button up and keeps the clip paused for visitors who ask for
/// reduced motion.
#[component]
pub fn Video() -> impl IntoView {
    view! {
        <div class="relative">
            <video
                autoplay
                muted
                loop
                playsinline
                id="gopalVideo"
                class="block aspect-[4/3] w-full bg-kraft object-cover lg:aspect-auto lg:h-[37.5rem]"
                poster="/assets/img/video/gopal_front.webp"
                aria-label="Vidéo du restaurant Le Gopal"
            >
                <source src="/assets/img/video/le_gopal_clip.mp4" type="video/mp4" />
            </video>
            <button
                data-video-toggle
                type="button"
                aria-controls="gopalVideo"
                aria-label="Mettre la vidéo en pause"
                data-label-pause="Mettre la vidéo en pause"
                data-label-play="Lire la vidéo"
                class="absolute bottom-4 left-4 flex size-14 items-center justify-center rounded-full bg-papier/90 text-encre shadow-sm hover:bg-papier lg:bottom-6 lg:left-6"
            >
                <svg data-video-icon="pause" class="size-[1.125rem]" viewBox="0 0 18 18" fill="currentColor" aria-hidden="true">
                    <rect x="3" y="2" width="4" height="14" />
                    <rect x="11" y="2" width="4" height="14" />
                </svg>
                <svg data-video-icon="play" class="hidden size-[1.125rem]" viewBox="0 0 18 18" fill="currentColor" aria-hidden="true">
                    <path d="M4 2l12 7-12 7z" />
                </svg>
            </button>
        </div>
    }
}

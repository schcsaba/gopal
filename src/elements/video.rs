use leptos::*;

#[component]
pub fn Video() -> impl IntoView {
    view! {
        <video autoplay class="absolute z-10 w-full h-full object-cover" id="gopalVideo" playsinline loop muted poster="assets/img/sunset.jpg">
            <source src="assets/img/video/le_gopal_clip.mp4" type="video/mp4" />
        </video>
    }
}

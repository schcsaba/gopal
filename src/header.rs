use crate::nav::Nav;
use crate::video::Video;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    let location = use_location();
    let is_home = move || location.pathname.get() == "/";

    view! {
        <header
            class=("h-screen", move || is_home())
            class=("h-auto", move || !is_home())
            class="relative flex w-full overflow-hidden"
        >
            <Nav/>
            {
                move || {
                    if is_home() {
                        Some(view! {<Video/>})
                    } else {
                        None
                    }
                }
            }
        </header>
    }
}

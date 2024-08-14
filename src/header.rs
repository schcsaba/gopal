use crate::nav::Nav;
use crate::video::Video;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="relative flex h-screen w-full overflow-hidden">
            <Nav/>
            <Video/>
        </header>
    }
}

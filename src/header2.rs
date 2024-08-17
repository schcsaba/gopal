use crate::nav2::Nav2;
use leptos::*;

#[component]
pub fn Header2() -> impl IntoView {
    view! {
        <header class="relative flex h-auto w-full overflow-hidden">
            <Nav2/>
        </header>
    }
}

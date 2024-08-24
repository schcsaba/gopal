use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Event;

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

#[server(GetImages)]
pub async fn get_images() -> Result<Vec<Image>, ServerFnError> {
    use std::fs;

    let mut images = Vec::new();
    let gallery_path = "public/assets/img/gallery";

    if let Ok(entries) = fs::read_dir(gallery_path) {
        for (id, entry) in entries.enumerate() {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".jpg") || filename.ends_with(".png") {
                        let aspect_ratio = 1.5;

                        images.push(Image {
                            id,
                            filename: filename.to_string(),
                            aspect_ratio,
                        });
                    }
                }
            }
        }
    }

    Ok(images)
}

#[component]
pub fn Gallery() -> impl IntoView {
    let images = create_resource(|| (), |_| get_images());
    let (visible_count, set_visible_count) = create_signal(20);
    let (expanded_image, set_expanded_image) = create_signal(None::<usize>);

    let handle_scroll = move |_: Event| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let scroll_top = window.scroll_y().unwrap_or(0.0);
        let window_height = window
            .inner_height()
            .unwrap_or(0.into())
            .as_f64()
            .unwrap_or(0.0);
        let document_height = document
            .document_element()
            .map(|el| el.scroll_height())
            .unwrap_or(0);

        if (document_height as f64) - (scroll_top + window_height) < 100.0 {
            set_visible_count.update(|count| *count += 10);
        }
    };

    on_cleanup({
        let window = web_sys::window().unwrap();
        let listener = Closure::wrap(Box::new(handle_scroll) as Box<dyn Fn(Event)>);
        window
            .add_event_listener_with_callback("scroll", listener.as_ref().unchecked_ref())
            .unwrap();
        move || {
            window
                .remove_event_listener_with_callback("scroll", listener.as_ref().unchecked_ref())
                .unwrap();
        }
    });

    let scroll_to_center = move |node_ref: NodeRef<html::Div>| {
        if let Some(element) = node_ref.get_untracked() {
            let window = web_sys::window().unwrap();
            let window_height = window
                .inner_height()
                .unwrap_or(0.into())
                .as_f64()
                .unwrap_or(0.0);

            let element: &web_sys::Element = element.as_ref();
            let rect = element.get_bounding_client_rect();
            let element_top = rect.top();
            let element_height = rect.height();

            let scroll_y = window.scroll_y().unwrap_or(0.0);
            let scroll_target = scroll_y + element_top - (window_height - element_height) / 2.0;
            window.scroll_with_x_and_y(0.0, scroll_target);
        }
    };

    view! {
        <div class="container mx-auto px-24 py-8">
            <Suspense
                fallback=move || view! {
                    <div class="flex justify-center items-center my-8">
                        <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-gray-900"></div>
                    </div>
                }
            >
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
                    {move || images.get().map(|images_result| {
                        match images_result {
                            Ok(images) => images.into_iter().take(visible_count.get()).map(|image| {
                                let id = image.id;
                                let is_expanded = move || expanded_image.get() == Some(id);
                                let node_ref = create_node_ref();
                                let toggle_expand = move |_| {
                                    if is_expanded() {
                                        set_expanded_image.set(None);
                                    } else {
                                        set_expanded_image.set(Some(id));
                                        request_animation_frame(move || {
                                            scroll_to_center(node_ref);
                                        });
                                    }
                                };

                                view! {
                                    <div
                                        _ref=node_ref
                                        class="relative overflow-hidden transition-all duration-700 ease-in-out cursor-pointer"
                                        class=("col-span-full", move || is_expanded())
                                        class=("hover:scale-105", move || !is_expanded())
                                        on:click=toggle_expand
                                    >
                                        <img
                                            src=format!("/assets/img/gallery/{}", image.filename)
                                            alt=format!("Image {}", image.id)
                                            class="w-full h-auto object-cover"
                                            style=format!("aspect-ratio: {}", image.aspect_ratio)
                                        />
                                    </div>
                                }
                            }).collect_view(),
                            Err(e) => view! { <div>"Error loading images: " {e.to_string()}</div> }.into_view(),
                        }
                    })}
                </div>
            </Suspense>
        </div>
    }
}

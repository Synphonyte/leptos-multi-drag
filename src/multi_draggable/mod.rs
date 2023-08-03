mod draggable;
pub use draggable::*;

use leptos::html::*;
use leptos::*;
use leptos_use::*;

#[component]
pub fn MultiDraggable() -> impl IntoView {
    let anchor = create_node_ref::<Div>();
    let drag_target_1 = create_node_ref::<Div>();
    let drag_target_2 = create_node_ref::<Div>();

    let draggable_items = vec!["one", "two", "three", "four", "five"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let draggable_items = create_rw_signal(draggable_items.clone());

    view! {
        <div node_ref=anchor class="drag_area" style="width: 100%; height: 100vh; background-color: green; text-align: center;">
            Droparea
            <div class="drop_zones" style="display: flex; margin: 10px;">
                <div class="drop_zone" style="background-color: red; width: 50%; height: 50%; text-align: center; margin: 10px;">
                    Dropzone
                    <For
                        each=move || draggable_items()
                        key=|k| k.to_string()
                        view=move |item: String| {
                            view! {
                                <Draggable drag_context_el=anchor drag_target_el=drag_target_1/>
                            }
                        }
                    />
                </div>
                <div class="drop_zone" style="background-color: red; width: 50%; height: 50%; text-align: center; margin: 10px;">
                    Dropzone
                    <For
                        each=move || draggable_items()
                        key=|k| k.to_string()
                        view=move |item: String| {
                            view! {
                                <Draggable drag_context_el=anchor drag_target_el=drag_target_2/>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

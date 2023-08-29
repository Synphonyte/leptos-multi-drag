use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, InputEvent};

use crate::multi_draggable::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let mut items = vec![];

    for i in 1..=100 {
        let key = format!("{}_{}_key", i, i);
        let value = format!("{}_{}", i, i);
        items.push((key, value));
    }

    let mut available_items = items
        .into_iter()
        .map(|(k, s)| DraggableItem {
            key: k,
            display_name: s,
        })
        .collect::<Vec<_>>();

    available_items.sort_by_key(|i| i.display_name.clone());

    let mut items = vec![];

    for i in 1..=100 {
        let key = format!("{}_key", i);
        let value = format!("{}", i);
        items.push((key, value));
    }

    let mut assigned_items = items
        .into_iter()
        .map(|(k, s)| DraggableItem {
            key: k,
            display_name: s,
        })
        .collect::<Vec<_>>();

    assigned_items.sort_by_key(|i| i.display_name.clone());

    let available_items = create_rw_signal(available_items.clone());
    let assigned_items = create_rw_signal(assigned_items.clone());

    let selected_item = create_rw_signal(None::<String>);

    let search_input = create_rw_signal("".to_string());

    let on_input = move |e: Event| {
        let event = e.unchecked_ref::<InputEvent>();
        search_input.update(|s| {
            if let Some(value) = event.data() {
                s.push_str(&value);
            } else {
                s.pop();
            }
        });
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! {
                    <input type="text" value="" on:input=on_input/>
                    <MultiDraggable available=available_items assigned=assigned_items selected_key=selected_item filter=search_input/>
                }/>
            </Routes>
        </Router>
    }
}

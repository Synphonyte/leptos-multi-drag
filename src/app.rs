use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{Event, InputEvent};

use crate::multi_draggable::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let mut available_items = vec![
        ("one_key", "one"),
        ("two_key", "two"),
        ("three_key", "three"),
        ("four_key", "four"),
        ("five_key", "five"),
    ]
    .into_iter()
    .map(|(k, s)| DraggableItem { key: k, content: s })
    .collect::<Vec<_>>();

    available_items.sort_by(|a, b| a.content.cmp(b.content));

    let mut assigned_items = vec![
        ("one_1_key", "one_1"),
        ("two_2_key", "two_2"),
        ("three_3_key", "three_3"),
        ("four_4_key", "four_4"),
        ("five_5_key", "five_5"),
    ]
    .into_iter()
    .map(|(k, s)| DraggableItem { key: k, content: s })
    .collect::<Vec<_>>();

    assigned_items.sort_by(|a, b| a.content.cmp(b.content));

    let available_items = create_rw_signal(available_items.clone());
    let assigned_items = create_rw_signal(assigned_items.clone());

    let available_selected_item = create_rw_signal(None::<&'static str>);
    let assigned_selected_item = create_rw_signal(None::<&'static str>);

    create_effect(move |_| {
        log!("available_items: {:?}", available_items.get());
    });

    create_effect(move |_| log!("assigned_items: {:?}", assigned_items.get()));

    let available = MultiDraggableList::Available(DraggableList {
        list: available_items,
        list_node_refs: create_rw_signal(HashMap::new()),
        selected_key: available_selected_item,
    });

    let assigned = MultiDraggableList::Assigned(DraggableList {
        list: assigned_items,
        list_node_refs: create_rw_signal(HashMap::new()),
        selected_key: assigned_selected_item,
    });

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
                    <MultiDraggable available=available assigned=assigned filter=search_input/>
                }/>
            </Routes>
        </Router>
    }
}

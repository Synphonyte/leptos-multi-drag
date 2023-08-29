mod draggable;

pub use draggable::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use leptos::html::*;
use leptos::*;
use web_sys::ScrollIntoViewOptions;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DraggableItem<K>
where
    K: Clone + Eq + Hash + 'static,
{
    pub key: K,
    pub display_name: String,
}

fn filter_list<K>(list: Vec<DraggableItem<K>>, search: String) -> Vec<DraggableItem<K>>
where
    K: Clone + Display + Eq + Hash + 'static,
{
    let mut filtered_list = list
        .clone()
        .into_iter()
        .filter(|item| item.display_name.starts_with(&search))
        .collect::<Vec<_>>();

    filtered_list.extend(
        list.into_iter()
            .filter(|item| {
                !item.display_name.starts_with(&search) && item.display_name[1..].contains(&search)
            })
            .collect::<Vec<_>>(),
    );

    filtered_list
}

#[component]
pub fn MultiDraggable<K>(
    available: RwSignal<Vec<DraggableItem<K>>>,
    assigned: RwSignal<Vec<DraggableItem<K>>>,
    selected_key: RwSignal<Option<K>>,
    filter: RwSignal<String>,
) -> impl IntoView
where
    K: Clone + Display + Eq + Hash + 'static,
{
    let anchor = create_node_ref::<Div>();

    let (cursor, set_cursor) = create_signal("normal");

    let drag_target_1 = create_node_ref::<Div>();
    let drag_target_2 = create_node_ref::<Div>();

    let highlight_target_1 = create_rw_signal(false);
    let highlight_target_2 = create_rw_signal(false);

    let get_style_target = move |highlight_signal: RwSignal<bool>| {
        move || {
            let border = if highlight_signal() {
                format!("border: 2px solid blue;")
            } else {
                format!("border: 2px solid transparent;")
            };

            format!(
                "background-color: red; \
                width: 50%; \
                height: 300px; \
                text-align: center; \
                margin: 10px; \
                cursor: inherit; \
                position: relative; \
                overflow-x: hidden; \
                overflow-y: scroll; \
                user-select: none;
                {border}",
            )
        }
    };

    let available_node_refs: RwSignal<HashMap<K, NodeRef<Div>>> = create_rw_signal(HashMap::new());
    let assigned_node_refs: RwSignal<HashMap<K, NodeRef<Div>>> = create_rw_signal(HashMap::new());

    let available_filtered = Signal::derive(move || filter_list(available(), filter()));
    let assigned_filtered = Signal::derive(move || filter_list(assigned(), filter()));

    let on_load = move |list: RwSignal<HashMap<K, NodeRef<Div>>>| {
        move |key: K, node_ref: NodeRef<Div>| {
            list.update(|map| {
                map.insert(key, node_ref);
            })
        }
    };

    let on_item_add =
        move |item: DraggableItem<K>,
              target_list: RwSignal<Vec<DraggableItem<K>>>,
              target_node_refs: RwSignal<HashMap<K, NodeRef<Div>>>| {
            target_list.update(|list| {
                let index = list
                    .binary_search_by(|i: &DraggableItem<K>| i.display_name.cmp(&item.display_name))
                    .unwrap_or_else(|i| i);

                list.insert(index, item.clone());

                selected_key.set(Some(item.key.clone()));
            });

            let node_ref = target_node_refs.get_untracked();
            let node_ref = node_ref.get(&item.key);

            if let Some(node_ref) = node_ref {
                if let Some(node_element) = node_ref.get_untracked() {
                    node_element.scroll_into_view_with_scroll_into_view_options(
                        ScrollIntoViewOptions::new().behavior(web_sys::ScrollBehavior::Smooth),
                    );
                }
            }
        };

    let on_item_remove =
        move |item: DraggableItem<K>,
              target_list: RwSignal<Vec<DraggableItem<K>>>,
              target_node_refs: RwSignal<HashMap<K, NodeRef<Div>>>| {
            target_list.update(|list| {
                list.retain(|i| i.key != item.key);
            });
            target_node_refs.update(|map| {
                map.remove(&item.key);
            });
        };

    view! {
        <div node_ref=anchor class="drag_area" style="width: 80%; height: 100%; margin: 25px; background-color: green; text-align: center; position: relative;" style:cursor=cursor>
            Droparea

            <div class="drop_zones" style="display: flex; margin: 10px; cursor: inherit;">
                <div class="drop_zone zone_1" node_ref=drag_target_1 style=get_style_target(highlight_target_1)>
                    Dropzone
                    <For
                        each=available_filtered
                        key=|item| format!("{}", item.key)
                        view=move |item| {
                            view! {
                                <Draggable
                                    id=item.key.clone()
                                    highlight_target=highlight_target_2
                                    container=anchor
                                    target_el=drag_target_2
                                    set_cursor=set_cursor
                                    selected=selected_key
                                    on_load=on_load(available_node_refs)
                                    on_item_add={
                                        let item = item.clone();
                                        move || on_item_add(item.clone(), assigned, assigned_node_refs)}
                                    on_item_remove={
                                        let item = item.clone();
                                        move || on_item_remove(item.clone(), available, available_node_refs)}
                                >
                                    {item.display_name}
                                </Draggable>
                            }
                        }
                    />
                </div>

                <div class="drop_zone zone_2" node_ref=drag_target_2 style=get_style_target(highlight_target_2)>
                    Dropzone
                    <For
                        each=assigned_filtered
                        key=|item| format!("{}", item.key)
                        view=move |item| {
                            view! {
                                <Draggable
                                    id=item.key.clone()
                                    highlight_target=highlight_target_1
                                    container=anchor
                                    target_el=drag_target_1
                                    set_cursor=set_cursor
                                    selected=selected_key
                                    on_load=on_load(assigned_node_refs)
                                    on_item_add={
                                        let item = item.clone();
                                        move || on_item_add(item.clone(), available, available_node_refs)}
                                    on_item_remove={
                                        let item = item.clone();
                                        move || on_item_remove(item.clone(), assigned, assigned_node_refs)}
                                >
                                    {item.display_name}
                                </Draggable>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

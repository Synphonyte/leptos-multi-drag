mod draggable;

pub use draggable::*;
use std::collections::HashMap;

use leptos::html::*;
use leptos::*;
use web_sys::ScrollIntoViewOptions;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DraggableItem {
    pub key: &'static str,
    pub content: &'static str,
}

#[derive(Copy, Clone)]
pub struct DraggableList {
    pub list: RwSignal<Vec<DraggableItem>>,
    pub list_node_refs: RwSignal<HashMap<&'static str, NodeRef<Div>>>,
    pub selected_key: RwSignal<Option<&'static str>>,
}
#[derive(Copy, Clone)]
pub enum MultiDraggableList {
    Available(DraggableList),
    Assigned(DraggableList),
}

impl MultiDraggableList {
    pub fn add(&self, item: DraggableItem) {
        let add = move |target_list: DraggableList, item: DraggableItem| {
            target_list.list.update(|list| {
                let index = list
                    .binary_search_by(|i: &DraggableItem| i.content.cmp(item.content))
                    .unwrap_or_else(|i| i);

                list.insert(index, item);

                self.set_selected_key(Some(item.key), false);
            });
        };

        match *self {
            MultiDraggableList::Available(draggable_list) => add(draggable_list, item),
            MultiDraggableList::Assigned(draggable_list) => add(draggable_list, item),
        }
    }

    pub fn remove(&self, item: DraggableItem) {
        self.get_list().update(|list| {
            list.retain(|i| i.key != item.key);
        });
        self.remove_list_node_ref(item.key);
    }

    pub fn filter(&self, search: String) -> Vec<DraggableItem> {
        log!("filter");
        let list = self.get_list().get();
        let mut filtered_list = list
            .clone()
            .into_iter()
            .filter(|item| item.content.starts_with(&search))
            .collect::<Vec<_>>();

        filtered_list.extend(
            list.into_iter()
                .filter(|item| {
                    !item.content.starts_with(&search) && item.content[1..].contains(&search)
                })
                .collect::<Vec<_>>(),
        );

        filtered_list
    }

    pub fn get_list(&self) -> RwSignal<Vec<DraggableItem>> {
        match *self {
            MultiDraggableList::Available(draggable_list) => draggable_list.list,
            MultiDraggableList::Assigned(draggable_list) => draggable_list.list,
        }
    }

    pub fn get_list_node_ref(&self, key: &'static str) -> NodeRef<Div> {
        match *self {
            MultiDraggableList::Available(draggable_list) => *draggable_list
                .list_node_refs
                .get_untracked()
                .get(key)
                .unwrap(),
            MultiDraggableList::Assigned(draggable_list) => *draggable_list
                .list_node_refs
                .get_untracked()
                .get(key)
                .unwrap(),
        }
    }

    pub fn add_list_node_ref(&self, key: &'static str, node_ref: NodeRef<Div>) {
        match *self {
            MultiDraggableList::Available(draggable_list) => {
                draggable_list.list_node_refs.update(|map| {
                    map.insert(key, node_ref);
                })
            }
            MultiDraggableList::Assigned(draggable_list) => {
                draggable_list.list_node_refs.update(|map| {
                    map.insert(key, node_ref);
                })
            }
        }
    }

    pub fn remove_list_node_ref(&self, key: &'static str) {
        match *self {
            MultiDraggableList::Available(draggable_list) => {
                draggable_list.list_node_refs.update(|map| {
                    map.remove(key);
                })
            }
            MultiDraggableList::Assigned(draggable_list) => {
                draggable_list.list_node_refs.update(|map| {
                    map.remove(key);
                })
            }
        }
    }

    pub fn selected_key(&self) -> RwSignal<Option<&'static str>> {
        match *self {
            MultiDraggableList::Available(draggable_list) => draggable_list.selected_key,
            MultiDraggableList::Assigned(draggable_list) => draggable_list.selected_key,
        }
    }

    pub fn set_selected_key(&self, key: Option<&'static str>, untracked: bool) {
        if untracked {
            match *self {
                MultiDraggableList::Available(draggable_list) => {
                    draggable_list.selected_key.set_untracked(key)
                }
                MultiDraggableList::Assigned(draggable_list) => {
                    draggable_list.selected_key.set_untracked(key)
                }
            }
        } else {
            match *self {
                MultiDraggableList::Available(draggable_list) => {
                    draggable_list.selected_key.set(key)
                }
                MultiDraggableList::Assigned(draggable_list) => {
                    draggable_list.selected_key.set(key)
                }
            };
        }
    }
}

#[component]
pub fn MultiDraggable(
    available: MultiDraggableList,
    assigned: MultiDraggableList,
    filter: RwSignal<String>,
) -> impl IntoView {
    let anchor = create_node_ref::<Div>();

    let (cursor, set_cursor) = create_signal("normal");
    let (is_dragging, set_dragging) = create_signal(false);

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

    let on_load = move |list: MultiDraggableList| {
        move |key: &'static str, node_ref: NodeRef<Div>| list.add_list_node_ref(key, node_ref)
    };

    let on_item_add = move |item: DraggableItem, target_list: MultiDraggableList| {
        target_list.add(item);

        let node_ref = target_list.get_list_node_ref(item.key);

        if let Some(node_element) = node_ref.get_untracked() {
            node_element.scroll_into_view_with_scroll_into_view_options(
                ScrollIntoViewOptions::new().behavior(web_sys::ScrollBehavior::Smooth),
            );
        }
    };

    let on_item_remove =
        move |item: DraggableItem, target_list: MultiDraggableList| target_list.remove(item);

    view! {
        <div node_ref=anchor class="drag_area" style="width: 80%; height: 100%; margin: 25px; background-color: green; text-align: center; position: relative;" style:cursor=cursor>
            Droparea

            <div class="drop_zones" style="display: flex; margin: 10px; cursor: inherit;">
                <div class="drop_zone zone_1" node_ref=drag_target_1 style=get_style_target(highlight_target_1)>
                    Dropzone
                    <For
                        each=move || available.filter(filter())
                        key=|item: &DraggableItem| format!("{}", item.key)
                        view=move |item: DraggableItem| {
                            view! {
                                <Draggable
                                    id=item.key
                                    highlight_target=highlight_target_2
                                    container=anchor
                                    target_el=drag_target_2
                                    set_cursor=set_cursor
                                    selected=available.selected_key()
                                    on_load=on_load(available)
                                    on_item_add=move || on_item_add(item, assigned)
                                    on_item_remove=move || on_item_remove(item, available)
                                >
                                    {item.content}
                                </Draggable>
                            }
                        }
                    />
                </div>

                <div class="drop_zone zone_2" node_ref=drag_target_2 style=get_style_target(highlight_target_2)>
                    Dropzone
                    <For
                        each=move || assigned.filter(filter())
                        key=|item: &DraggableItem| format!("{}", item.key)
                        view=move |item: DraggableItem| {
                            view! {
                                <Draggable
                                    id=item.key
                                    highlight_target=highlight_target_1
                                    container=anchor
                                    target_el=drag_target_1
                                    set_cursor=set_cursor
                                    selected=assigned.selected_key()
                                    on_load=on_load(assigned)
                                    on_item_add=move || on_item_add(item, available)
                                    on_item_remove=move || on_item_remove(item, assigned)
                                >
                                    {item.content}
                                </Draggable>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

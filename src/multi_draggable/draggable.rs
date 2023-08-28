use leptos::html::{Div};
use leptos::*;
use leptos_use::core::Position;
use leptos_use::*;
use std::time::Duration;
use wasm_bindgen::JsCast;

#[component]
pub fn Draggable<OnLoad, OnAdd, OnRm>(
    id: &'static str,
    highlight_target: RwSignal<bool>,
    container: NodeRef<Div>,
    target_el: NodeRef<Div>,
    set_cursor: WriteSignal<&'static str>,
    selected: RwSignal<Option<&'static str>>,
    on_load: OnLoad,
    on_item_add: OnAdd,
    on_item_remove: OnRm,
    children: Children,
) -> impl IntoView
where
    OnLoad: Fn(&'static str, NodeRef<Div>) + Clone + 'static,
    OnAdd: Fn() + Clone + 'static,
    OnRm: Fn() + Clone + 'static,
{
    let el_ref = create_node_ref::<Div>();

    let (el_ref_clone, set_el_ref_clone) = create_signal(None::<web_sys::Element>);

    let drag_offset = create_rw_signal(Position::default());

    let (transition, set_transition) = create_signal("");
    let (dragging_style, set_dragging_style) = create_signal(String::new());
    let (dragging_proxy_style, set_dragging_proxy_style) = create_signal("");

    let (is_target_hovered, set_target_hovered) = create_signal(false);

    let initial_value = create_rw_signal(Position::default());

    let is_self_hovered = use_element_hover(el_ref);

    on_load(id, el_ref);

    let on_drag_start = move |args: UseDraggableCallbackArgs| {
        let UseDraggableCallbackArgs { position, event } = args;

        selected.set(Some(id));

        if let Some(html_el) = el_ref.get_untracked() {
            let element = html_el.unchecked_ref::<web_sys::Element>();

            if let Ok(el_cloned) = element.clone_node_with_deep(true) {
                if let Some(container) = container.get_untracked() {
                    container.append_child(&el_cloned).ok();
                };
                set_el_ref_clone(Some(el_cloned.unchecked_into()));
            }
        }

        let container_rect = container
            .get_untracked()
            .expect("not empty")
            .get_bounding_client_rect();

        let el_rect = el_ref
            .get_untracked()
            .expect("not empty")
            .get_bounding_client_rect();

        initial_value.set(Position {
            x: event.x() as f64 - position.x,
            y: event.y() as f64 - position.y,
        });

        drag_offset.set(Position {
            x: container_rect.left(),
            y: container_rect.top(),
        });

        set_dragging_style(format!(
            "
                position: absolute; \
                pointer-events: none; \
                z-index: 100; \
                user-select: none; \
                top: 0px; \
                left: 0px;\
                width: {}px; \
                height: {}px;
            ",
            el_rect.width(),
            el_rect.height()
        ));

        set_cursor("grabbing");

        set_transition("");

        set_dragging_proxy_style(
            "\
                cursor: grabbing;\
                pointer-events: none;\
                user-select: none;\
                visibility: hidden;\
            ",
        );

        true
    };

    let on_drag_move = move |args: UseDraggableCallbackArgs| {
        if let Some(target_el) = target_el.get_untracked() {
            let UseDraggableCallbackArgs { position, event } = args;

            let mut target = Some(event_target::<web_sys::Element>(&event));

            while let Some(t) = &target {
                if t == target_el.unchecked_ref::<web_sys::Element>() {
                    break;
                }

                target = t.parent_element();
            }

            set_target_hovered(target.is_some());
        }
    };

    let on_drag_end = move |args: UseDraggableCallbackArgs| {
        let UseDraggableCallbackArgs { position, event } = args;

        if is_target_hovered.get_untracked() {
            let element = el_ref_clone
                .get_untracked()
                .expect("draggable element to be present");

            element.remove();
            set_el_ref_clone(None);

            on_item_add();

            let on_item_remove = on_item_remove.clone();
            on_item_remove();
        } else {
            let element = el_ref
                .get_untracked()
                .expect("draggable element to be present");
            let rect = element.get_bounding_client_rect();

            set_transition("transition: transform .3s cubic-bezier(0.2, 1, 0.1, 1); ");
            initial_value.set(Position {
                x: rect.left(),
                y: rect.top(),
            });

            set_timeout(
                move || {
                    set_transition("");
                    set_dragging_proxy_style("pointer-events: auto;");

                    let element = el_ref_clone
                        .get_untracked()
                        .expect("draggable element to be present");
                    element.remove();

                    set_el_ref_clone(None);
                },
                Duration::from_millis(300),
            );
        }
    };

    let UseDraggableReturn {
        x, y, is_dragging, ..
    } = use_draggable_with_options(
        el_ref,
        UseDraggableOptions::default()
            .on_start(on_drag_start)
            .on_move(on_drag_move)
            .on_end(on_drag_end)
            .initial_value(initial_value)
            .prevent_default(false),
    );

    let relative_position = Signal::derive(move || Position {
        x: x() - drag_offset().x,
        y: y() - drag_offset().y,
    });

    let log_rel_x = Signal::derive(move || format!("X: {}", relative_position().x));

    let log_rel_y = Signal::derive(move || format!("Y: {}", relative_position().y));

    let hover_style = Signal::derive(move || {
        if is_self_hovered() || is_dragging() {
            "border: 2px solid blue;".to_string()
        } else {
            "border: 2px solid transparent;".to_string()
        }
    });

    let selected_style = Signal::derive(move || {
        if let Some(key) = selected() {
            if key == id {
                "background-color: blue;".to_string()
            } else {
                "background-color: white;".to_string()
            }
        } else {
            "background-color: white;".to_string()
        }
    });

    let style = Signal::derive(move || {
        format!(
            "{}{}{}transform: translate({}px, {}px); cursor: grab;",
            dragging_style(),
            transition(),
            hover_style(),
            relative_position().x,
            relative_position().y,
        )
    });

    let proxy_style =
        Signal::derive(move || format!("{}{}", dragging_proxy_style(), hover_style()));

    let _ = leptos::watch(
        move || is_target_hovered() && is_dragging(),
        move |highlight, _, _| {
            highlight_target.set(*highlight);
        },
        false,
    );

    create_effect(move |_| {
        if let Some(drag_element) = el_ref_clone() {
            drag_element.set_attribute("style", &style()).ok();
        }
    });

    view! {
        <div id=move || format!("draggable-{id}") class="draggable" node_ref=el_ref style=move || format!("width: 100%; height: fit-content; user-select: none; {}", proxy_style())>
            <h1 class="content" style=move || format!("{} width: 100%; user-select-none;", selected_style())>
                {children()}
                <div>{log_rel_x}</div>
                <div>{log_rel_y}</div>
            </h1>

        </div>
    }
}

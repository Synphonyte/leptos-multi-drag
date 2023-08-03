use leptos::html::Div;
use leptos::*;
use leptos_use::*;

#[component]
pub fn Draggable(drag_context_el: NodeRef<Div>, drag_target_el: NodeRef<Div>) -> impl IntoView {
    let el = create_node_ref::<Div>();

    let position_relative_to_parent = |args: UseDraggableCallbackArgs| {
        let UseDraggableCallbackArgs { position, event } = args;

        log!("{}, {}", position.x, position.y);
        log!("{event:?}");

        true
    };

    let UseDraggableReturn { x, y, .. } = use_draggable_with_options(
        el,
        UseDraggableOptions::default()
            .dragging_element(drag_context_el)
            .on_start(position_relative_to_parent)
            .prevent_default(true),
    );

    let style =
        Signal::derive(move || format!("position: relative; left: {}px; top: {}px;", x(), y()));

    view! {
        <div node_ref=el style=move || format!("width: fit-content; height: fit-content; {}", style())>
            <h1 class="draggable" style="background-color: white;">
                Drag me!
                <h5>x: {x} y: {y}</h5>
            </h1>
        </div>
    }
}

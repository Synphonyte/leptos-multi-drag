use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::multi_draggable::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! {  <MultiDraggable/> }/>
            </Routes>
        </Router>
    }
}

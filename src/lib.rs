use leptos::*;
use wasm_bindgen::prelude::*;

mod components;
mod ckad_data;

use components::CkadWiki;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <CkadWiki />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

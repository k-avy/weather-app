use dominator::html;
use wasm_bindgen::prelude::*;

mod app_state;
mod location_input;
mod routes;
#[wasm_bindgen(start)]
pub fn main_js() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    dominator::append_dom(&dominator::body(), html!("div", {
        .child(html!("h1", {
            .text("Weather App")
        }))
    }));

   let loc=location_input::LocationInput::new();
    location_input::LocationInput::render(&loc);

}
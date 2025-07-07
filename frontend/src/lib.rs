use wasm_bindgen::prelude::*;

mod simple_app;

use simple_app::SimpleApp;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<SimpleApp>::new().render();
}

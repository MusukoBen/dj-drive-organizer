use wasm_bindgen::prelude::*;

mod minimal_drive_hooks;
mod simple_demo;
mod tauri_api;
mod types;

use simple_demo::SimpleDriveDemo;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<SimpleDriveDemo>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
use dj_drive_organizer::components::App;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This is a WASM-only application. Use 'trunk serve' to run it.");
}

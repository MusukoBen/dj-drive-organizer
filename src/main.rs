#[cfg(not(target_arch = "wasm32"))]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This is a WASM-only application. Use 'trunk serve' to run it.");
}

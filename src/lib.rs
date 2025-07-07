// Main library crate that re-exports frontend functionality
pub use dj_drive_organizer_frontend::components;

// For WASM builds, use the frontend's wasm_main
#[cfg(target_arch = "wasm32")]
pub use dj_drive_organizer_frontend::wasm_main;

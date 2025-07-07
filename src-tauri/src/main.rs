// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

#[cfg(target_os = "macos")]
mod fs_macos;

#[cfg(target_os = "windows")]
mod fs_windows;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::scan_drive,
            commands::get_file_metadata
        ])
        .run(tauri::generate_context!())
        // Hinweis: Stelle sicher, dass die Datei unter src-tauri/icons/icon.png im RGBA-Format vorliegt
        // Du kannst das Bild mit einem Bildbearbeitungsprogramm wie GIMP oder ImageMagick konvertieren
        .expect("error while running tauri application");
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_udp::init())
        .run(ctx)
        .expect("error while running tauri application");
}

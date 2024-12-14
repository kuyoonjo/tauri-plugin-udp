use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;
pub mod error;
pub mod models;

pub mod platform;

const PLUGIN_NAME: &str = "udp";

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            commands::bind,
            commands::unbind,
            commands::send
        ])
        .build()
}

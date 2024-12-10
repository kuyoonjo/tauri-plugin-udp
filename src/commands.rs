use tauri::Runtime;

use crate::error::Result;

use crate::platform;


#[tauri::command]
pub async fn bind<R: Runtime>(
    window: tauri::Window<R>,
    id: String,
    bind_at: String,
    broadcast: Option<bool>,
) -> Result<()> {
    platform::bind(window, id, bind_at, broadcast.is_some_and(|v| v)).await.map_err(|e| e.into())
}

#[tauri::command]
pub async fn unbind(id: String) -> Result<()> {
    platform::unbind(id).await.map_err(|e| e.into())
}

#[tauri::command]
pub async fn send(id: String, target: String, message: Vec<u8>) -> Result<()> {
    platform::send(id, target, message).await.map_err(|e| e.into())
}

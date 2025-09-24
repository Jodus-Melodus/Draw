use tauri::AppHandle;

use crate::file::open_file;

pub async fn add_track_file(app_handle: &AppHandle) {
    open_file(app_handle).await;
}

pub fn add_track_stream(app: &AppHandle) {}

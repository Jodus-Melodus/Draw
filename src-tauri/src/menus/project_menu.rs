use tauri::AppHandle;

use crate::{file::open_file, pages::select_input_stream::open_select_input_stream};

pub async fn add_track_file(app_handle: &AppHandle) {
    open_file(app_handle).await;
}

pub fn add_track_stream(app_handle: &AppHandle) {
    open_select_input_stream(app_handle);
}

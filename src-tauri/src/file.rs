use std::path::PathBuf;

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

use crate::{
    states,
    track
};

pub async fn open_file(app_handle: &AppHandle) {
    let mixer_state = app_handle.state::<states::StateMixer>();
    let state = mixer_state.clone();
    let file_path = select_file(app_handle).await;
    if let Some(path) = file_path {
        let track_list = state.track_list.clone();
        let mut list = track_list.lock().expect("Failed to lock track list");
        let track_source = track::FileSource::new_input(&path);
        let track = track::Track::new(track::TrackType::In, track::TrackAudioSource::File(track_source));
        list.add_track(
            &path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .expect("Failed to get file name"),
            track,
        );
    } else {
        eprintln!("No file selected");
    }
}

async fn select_file(app_handle: &AppHandle) -> Option<PathBuf> {
    let file_path = app_handle.dialog().file().blocking_pick_file();
    file_path.map(|p| p.into_path().unwrap_or_default())
}

use std::sync::atomic::Ordering;

use tauri::AppHandle;

use crate::{
    file::open_file,
    states,
    track::{StreamSource, Track, TrackType},
};

pub async fn add_track_file(app_handle: &AppHandle) {
    open_file(app_handle).await;
}

#[tauri::command]
pub fn add_track_stream(
    audio_context: tauri::State<states::StateAudioContext>,
    mixer: tauri::State<states::StateMixer>,
) {
    let device = audio_context
        .input_device()
        .expect("Failed to get input device");
    let list = mixer.track_list.clone();
    let mut track_list = list.lock().expect("Failed to lock track list");
    let source = StreamSource::new(device.clone());
    let track = Track::new(TrackType::In, Box::new(source));
    track_list.add_track("testing-1-2", track);
}

#[tauri::command]
pub fn select_input_stream(
    audio_context: tauri::State<states::StateAudioContext>,
    device_index: usize,
) {
    let input_index = audio_context.input_device_index.clone();
    input_index.store(device_index, Ordering::Relaxed);
}

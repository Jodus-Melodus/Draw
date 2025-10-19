use std::sync::{Arc, Mutex};

use crate::{project, track};

#[tauri::command]
pub fn add_empty_track(
    mixer: tauri::State<project::states::StateMixerGuard>,
    audio: tauri::State<project::states::StateAudioContext>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let state_mixer = mixer.0.lock().map_err(|_| "Failed to lock state mixer")?;
    let audio_context = audio.clone();
    let list = state_mixer.track_list.clone();
    let mut track_list = list.lock().map_err(|_| "Failed to lock track list")?;
    if let Some(input_device) = audio_context.input_device() {
        let number = track_list.track_list().len() + 1;
        let new_track_name = format!("track-{}", number);
        let path = format!("{}.wav", new_track_name);
        let stream_source = track::source::StreamSource::new(&app, input_device);
        let file_source = track::source::FileSource::new(path.into(), stream_source.sample_rate);
        let track = track::track::AudioTrack::new(
            &new_track_name,
            track::track::TrackType::In,
            Some(stream_source),
            Arc::new(Mutex::new(file_source)),
        );

        let new_track_name = format!("track-{}", number);
        track_list.add_track(&new_track_name, track);
    }
    Ok(())
}

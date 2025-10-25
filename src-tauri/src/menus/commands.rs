use crate::{project, track};

#[tauri::command]
pub fn add_empty_track(
    mixer: tauri::State<project::states::StateMixerGuard>,
    audio: tauri::State<project::states::StateAudioContext>,
) -> Result<(), String> {
    let state_mixer = mixer.0.lock().map_err(|_| "Failed to lock state mixer")?;
    let audio_context = audio.clone();
    let list = state_mixer.track_list.clone();
    let mut track_list = list.lock().map_err(|_| "Failed to lock track list")?;
    if let Some(input_device) = audio_context.input_device() {
        let number = track_list.track_list().len() + 1;
        let name = format!("track-{}", number);
        let source = track::sources::source::StreamSource::new(input_device);
        let track = track::tracks::InputTrack::new(&name, Box::new(source));
        let new_track_name = format!("track-{}", number);
        track_list.add_track(&new_track_name, track);
    }
    Ok(())
}

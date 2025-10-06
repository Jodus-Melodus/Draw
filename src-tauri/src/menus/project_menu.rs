use crate::{states, track};

#[tauri::command]
pub fn add_empty_track(mixer: tauri::State<states::StateMixerGuard>) -> Result<(), String> {
    let state_mixer = mixer.0.lock().map_err(|_| "Failed to lock state mixer")?;
    let list = state_mixer.track_list.clone();
    let mut track_list = list.lock().map_err(|_| "Failed to lock track list")?;

    let track = track::AudioTrack::new(track::TrackType::In, None, None);
    let new_track_name = format!("track-{}", track_list.track_list().len() + 1);
    track_list.add_track(&new_track_name, track);
    Ok(())
}

use crate::{project, track};

#[tauri::command]
pub fn get_track_list(
    state: tauri::State<project::states::StateMixerGuard>,
) -> Result<track::track_list::TrackListResponse, String> {
    let state_mixer = state.0.lock().map_err(|_| "Failed to lock state mixer")?;
    let master_out = state_mixer
        .master_out
        .lock()
        .map_err(|_| "Failed to lock track list")?;
    let list = state_mixer
        .track_list
        .lock()
        .map_err(|_| "Failed to lock track list")?;
    let mut response = list.as_response();
    response.tracks.insert(0, master_out.as_response());
    Ok(response)
}

#[tauri::command]
pub fn update_track(
    state: tauri::State<project::states::StateMixerGuard>,
    track_name: String,
    update: track::track_list::TrackUpdate,
) -> Result<(), String> {
    let state_mixer = state.0.lock().map_err(|_| "Failed to lock state mixer")?;

    if track_name == "master-out" {
        let master_out = state_mixer.master_out.clone();
        if let Ok(mut output) = master_out.lock() {
            match update {
                track::track_list::TrackUpdate::Pan(pan) => output.pan = pan,
                track::track_list::TrackUpdate::Gain(gain) => output.gain = gain,
                _ => (),
            }

            println!("Master out gain: {}", output.gain);
        };
    } else {
        let mut list = state_mixer
            .track_list
            .lock()
            .map_err(|_| "Failed to lock track list")?;
        list.update_track(&track_name, update);
    }
    Ok(())
}

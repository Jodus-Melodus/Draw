use crate::{states, track};

#[tauri::command]
pub fn add_empty_track(mixer: tauri::State<states::StateMixer>) {
    let list = mixer.track_list.clone();
    let mut track_list = list.lock().expect("Failed to lock track list");
    let track = track::AudioTrack::new(track::TrackType::In, None, None);
    track_list.add_track("testing-1-2", track);
    println!("Added empty track");
}

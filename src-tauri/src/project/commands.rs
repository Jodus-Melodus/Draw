// pub fn save_project(app_handle: &AppHandle) {
//     let app = app_handle.clone();

//     app.dialog()
//         .file()
//         .set_title("Save project")
//         .pick_folder(move |folder_path| {
//             if let Some(folder_path) = folder_path {
//                 if let Some(path) = folder_path.as_path() {
//                     let config = bincode::config::standard();

//                     let mixer_state_path = path.join("mixer_state.mix");
//                     let state_mixer_guard = app.state::<project::states::StateMixerGuard>();
//                     let mixer_guard = state_mixer_guard.0.lock().unwrap();
//                     let raw_state_mixer = project::states::StateMixerRaw::from(mixer_guard.clone());
//                     let encoded_mixer = bincode::encode_to_vec(&raw_state_mixer, config).unwrap();

//                     let mut file = File::create(&mixer_state_path).expect(&format!(
//                         "Failed to create file {}",
//                         mixer_state_path.display()
//                     ));
//                     file.write_all(&encoded_mixer).expect(&format!(
//                         "Failed to write to file {}",
//                         mixer_state_path.display()
//                     ));
//                 }
//             }
//         });
// }

// #[tauri::command]
// pub fn load_project(app_handle: &AppHandle) {
//     let app = app_handle.clone();

//     app.dialog()
//         .file()
//         .set_title("Open project")
//         .pick_folder(move |folder_path| {
//             if let Some(folder_path) = folder_path {
//                 if let Some(path) = folder_path.as_path() {
//                     let config = bincode::config::standard();

//                     let mixer_state_path = path.join("mixer_state.mix");
//                     let mut mixer_state_file = File::open(&mixer_state_path).expect(&format!(
//                         "Failed to open file {}",
//                         mixer_state_path.display()
//                     ));
//                     let mut mixer_state_buffer = Vec::new();
//                     mixer_state_file
//                         .read_to_end(&mut mixer_state_buffer)
//                         .expect(&format!(
//                             "Failed to read file {}",
//                             mixer_state_path.display()
//                         ));
//                     let (decoded_mixer, _): (project::states::StateMixerRaw, usize) =
//                         bincode::decode_from_slice(&mixer_state_buffer, config).unwrap();
//                     let new_state_mixer = project::states::StateMixer::from(decoded_mixer);
//                     let state_mixer = app.state::<project::states::StateMixerGuard>();
//                     let mut guard = state_mixer.0.lock().unwrap();
//                     *guard = new_state_mixer;

//                     let window = app
//                         .get_webview_window("main")
//                         .expect("Failed to get main window");
//                     window.emit("updated-track-list", ()).unwrap();
//                 }
//             }
//         });
// }

use tauri::State;

use crate::project;

#[tauri::command]
pub fn start_stream(mixer: State<project::states::StateMixerGuard>) {
    let mixer_state = mixer.0.lock().unwrap();
    let track_list = mixer_state.track_list.lock().unwrap();

    for track in track_list.get_tracks() {
        if let Ok(t) = track.lock() {
            t.source.start_stream();
        } else {
            eprintln!("Failed to lock track");
        }
    }
}

#[tauri::command]
pub fn stop_stream(mixer: State<project::states::StateMixerGuard>) {
    let mixer_state = mixer.0.lock().unwrap();
    let track_list = mixer_state.track_list.lock().unwrap();

    for track in track_list.get_tracks() {
        if let Ok(t) = track.lock() {
            t.source.stop_stream();
        } else {
            eprintln!("Failed to lock track");
        }
    }
}

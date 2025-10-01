use std::{fs::File, io::Write};

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

use crate::states;

pub fn save_project(app_handle: &AppHandle) {
    let app = app_handle.clone();

    app.dialog()
        .file()
        .set_title("Save project")
        .pick_folder(move |folder_path| {
            if let Some(folder_path) = folder_path {
                if let Some(path) = folder_path.as_path() {
                    let config = bincode::config::standard();

                    let mixer_state_path = path.join("mixer_state.mix");
                    let mixer_state = app.state::<states::StateMixer>().inner().clone();
                    let encoded_mixer =
                        bincode::encode_to_vec(&mixer_state.to_raw(), config).unwrap();
                    let mut file = File::create(&mixer_state_path)
                        .expect(&format!("Failed to create file {:?}", mixer_state_path));
                    file.write_all(&encoded_mixer)
                        .expect(&format!("Failed to write to file {:?}", mixer_state_path));
                }
            }
        });
}

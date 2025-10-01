use std::{
    fs::File,
    io::{Read, Write},
};

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
                    let raw_state_mixer = states::StateMixerRaw::from(mixer_state);
                    let encoded_mixer = bincode::encode_to_vec(&raw_state_mixer, config).unwrap();
                    let mut file = File::create(&mixer_state_path).expect(&format!(
                        "Failed to create file {}",
                        mixer_state_path.display()
                    ));
                    file.write_all(&encoded_mixer).expect(&format!(
                        "Failed to write to file {}",
                        mixer_state_path.display()
                    ));
                }
            }
        });
}

pub fn load_project(app_handle: &AppHandle) {
    let app = app_handle.clone();

    app.dialog()
        .file()
        .set_title("Open project")
        .pick_folder(move |folder_path| {
            if let Some(folder_path) = folder_path {
                if let Some(path) = folder_path.as_path() {
                    let config = bincode::config::standard();

                    let mixer_state_path = path.join("mixer_state.mix");
                    let mut mixer_state_file = File::open(&mixer_state_path).expect(&format!(
                        "Failed to open file {}",
                        mixer_state_path.display()
                    ));
                    let mut mixer_state_buffer = Vec::new();
                    mixer_state_file
                        .read_to_end(&mut mixer_state_buffer)
                        .expect(&format!(
                            "Failed to read file {}",
                            mixer_state_path.display()
                        ));
                    let (decoded_mixer, _len): (states::StateMixerRaw, usize) =
                        bincode::decode_from_slice(&mixer_state_buffer, config).unwrap();
                    let new_state_mixer = states::StateMixer::from(decoded_mixer);
                    // let state_mixer = app.state::<states::StateMixer>();
                    // let mut inner_mixer_state = state_mixer.inner().clone();
                    // *inner_mixer_state = states::StateMixer::from_raw(decoded_mixer);

                    app.manage(new_state_mixer);
                }
            }
        });
}

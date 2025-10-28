use std::{
    fs::File,
    io::{Read, Write},
};

use bincode::config::Configuration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use crate::project;

#[tauri::command]
pub fn save_project(app_handle: AppHandle) {
    app_handle
        .dialog()
        .file()
        .set_title("Save project")
        .pick_folder(move |folder_path| {
            if let Some(folder_path) = folder_path {
                if let Some(path) = folder_path.as_path() {
                    let config = bincode::config::standard();
                    let mixer_state_path = path.join("mixer_state.mix");
                    let state_mixer_guard = app_handle.state::<project::states::StateMixerGuard>();

                    if let Ok(mixer_guard) = state_mixer_guard.0.lock() {
                        let raw_state_mixer = project::states::StateMixerRaw::from(&*mixer_guard);
                        if let Ok(encoded_mixer) = bincode::encode_to_vec(&raw_state_mixer, config)
                        {
                            if let Ok(mut file) = File::create(&mixer_state_path) {
                                if let Ok(_) = file.write_all(&encoded_mixer) {
                                } else {
                                    app_handle
                                        .dialog()
                                        .message("Failed to write to file")
                                        .title("File Error")
                                        .kind(MessageDialogKind::Warning)
                                        .buttons(MessageDialogButtons::Ok)
                                        .blocking_show();
                                }
                            } else {
                                app_handle
                                    .dialog()
                                    .message("Failed to create file")
                                    .title("File Error")
                                    .kind(MessageDialogKind::Warning)
                                    .buttons(MessageDialogButtons::Ok)
                                    .blocking_show();
                            }
                        } else {
                            app_handle
                                .dialog()
                                .message("Failed to encode state mixer")
                                .title("Encode Error")
                                .kind(MessageDialogKind::Warning)
                                .buttons(MessageDialogButtons::Ok)
                                .blocking_show();
                        }
                    } else {
                        app_handle
                            .dialog()
                            .message("Failed to lock state mixer")
                            .title("State Error")
                            .kind(MessageDialogKind::Warning)
                            .buttons(MessageDialogButtons::Ok)
                            .blocking_show();
                    };
                } else {
                    app_handle
                        .dialog()
                        .message("Invalid path")
                        .title("Path Error")
                        .kind(MessageDialogKind::Warning)
                        .buttons(MessageDialogButtons::Ok)
                        .blocking_show();
                }
            } else {
                app_handle
                    .dialog()
                    .message("No destination selected")
                    .title("Destination")
                    .kind(MessageDialogKind::Info)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
            }
        });
}

#[tauri::command]
pub fn load_project(app_handle: AppHandle) {
    app_handle
        .dialog()
        .file()
        .set_title("Open project")
        .pick_folder(move |folder_path| {
            if let Some(folder_path) = folder_path {
                if let Some(path) = folder_path.as_path() {
                    let config = bincode::config::standard();
                    let mixer_state_path = path.join("mixer_state.mix");

                    if let Ok(mut mixer_state_file) = File::open(&mixer_state_path) {
                        let mut mixer_state_buffer = Vec::new();
                        if let Ok(_) = mixer_state_file.read_to_end(&mut mixer_state_buffer) {
                            if let Ok((decoded_mixer, _)) =
                                bincode::decode_from_slice::<
                                    project::states::StateMixerRaw,
                                    Configuration,
                                >(&mixer_state_buffer, config)
                            {
                                let new_state_mixer =
                                    project::states::StateMixer::from(decoded_mixer);
                                let state_mixer =
                                    app_handle.state::<project::states::StateMixerGuard>();

                                if let Ok(mut guard) = state_mixer.0.lock() {
                                    *guard = new_state_mixer;

                                    let window = app_handle
                                        .get_webview_window("main")
                                        .expect("Failed to get main window");
                                    window.emit("updated-track-list", ()).unwrap();
                                };
                            } else {
                                app_handle
                                    .dialog()
                                    .message("Failed to decode state mixer")
                                    .title("Decode Error")
                                    .kind(MessageDialogKind::Warning)
                                    .buttons(MessageDialogButtons::Ok)
                                    .blocking_show();
                            }
                        } else {
                            app_handle
                                .dialog()
                                .message("Failed to read file")
                                .title("File Error")
                                .kind(MessageDialogKind::Warning)
                                .buttons(MessageDialogButtons::Ok)
                                .blocking_show();
                        }
                    } else {
                        app_handle
                            .dialog()
                            .message("Failed to open file")
                            .title("File Error")
                            .kind(MessageDialogKind::Warning)
                            .buttons(MessageDialogButtons::Ok)
                            .blocking_show();
                    }
                } else {
                    app_handle
                        .dialog()
                        .message("Invalid path")
                        .title("Path Error")
                        .kind(MessageDialogKind::Warning)
                        .buttons(MessageDialogButtons::Ok)
                        .blocking_show();
                }
            } else {
                app_handle
                    .dialog()
                    .message("No destination selected")
                    .title("Destination")
                    .kind(MessageDialogKind::Info)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
            }
        });
}

#[tauri::command]
pub fn start_stream(app_handle: AppHandle) {
    app_handle
        .dialog()
        .message("Feature not implemented yet")
        .title("Error")
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::Ok)
        .blocking_show();
}

#[tauri::command]
pub fn stop_stream(app_handle: AppHandle) {
    app_handle
        .dialog()
        .message("Feature not implemented yet")
        .title("Error")
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::Ok)
        .blocking_show();
}

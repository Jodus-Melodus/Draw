use std::path::PathBuf;

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;

use crate::{states, track};

pub async fn open_files(app_handle: &AppHandle) {
    let app = app_handle.clone();

    app.dialog()
        .file()
        .set_title("Select a file to open")
        .add_filter("WAV files", &["wav"])
        .pick_files(move |paths| {
            let file_paths = paths.map(|vec| {
                vec.into_iter()
                    .map(|p| p.into_path().unwrap_or_default())
                    .collect::<Vec<_>>()
            });

            if let Some(paths) = file_paths {
                let state_mixer_guard = app.state::<states::StateMixerGuard>();
                for path in paths {
                    if let Some(extention) = path
                        .extension()
                        .expect("Failed to get file extention")
                        .to_str()
                    {
                        match extention {
                            "wav" => add_file_track(&app, state_mixer_guard.clone(), path.clone()),
                            _ => eprintln!("Unsuppored file extention: {}", extention),
                        }
                    }
                }
            } else {
                eprintln!("No files selected");
            }
        });
}

fn add_file_track(
    app_handle: &AppHandle,
    state_mixer_guard: tauri::State<states::StateMixerGuard>,
    path: PathBuf,
) {
    let state_mixer = state_mixer_guard.0.lock().unwrap();
    let track_list = state_mixer.track_list.clone();
    let mut list = track_list.lock().expect("Failed to lock track list");
    let track_source = track::FileSource::new(&path);
    let track = track::AudioTrack::new(track::TrackType::In, None, Some(track_source));
    list.add_track(
        &path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .expect("Failed to get file name"),
        track,
    );

    let window = app_handle
        .get_webview_window("main")
        .expect("Failed to get main window");
    window.emit("updated-track-list", ()).unwrap();
}

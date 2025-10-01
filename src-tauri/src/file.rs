use std::path::PathBuf;

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

use crate::{states, track};

pub async fn open_files(app_handle: &AppHandle) {
    let app = app_handle.clone();
    let mixer_state = app.state::<states::StateMixer>().inner().clone();

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
                for path in paths {
                    if let Some(extention) = path
                        .extension()
                        .expect("Failed to get file extention")
                        .to_str()
                    {
                        match extention {
                            "wav" => add_file_track(mixer_state.clone(), path.clone()),
                            _ => eprintln!("Unsuppored file extention: {}", extention),
                        }
                    }
                }
            } else {
                eprintln!("No files selected");
            }
        });
}

fn add_file_track(state: states::StateMixer, path: PathBuf) {
    let track_list = state.track_list.clone();
    let mut list = track_list.lock().expect("Failed to lock track list");
    let track_source = track::FileSource::new_input(&path);
    let track = track::AudioTrack::new(track::TrackType::In, None, Some(track_source));
    list.add_track(
        &path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .expect("Failed to get file name"),
        track,
    );

    println!("{:?}", list.track_list());
}

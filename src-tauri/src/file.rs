use std::path::PathBuf;

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;

use crate::{states, track};

pub async fn open_file(app_handle: &AppHandle) {
    let mixer_state = app_handle.state::<states::StateMixer>();
    let file_paths = select_file(app_handle).await;

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
}

pub async fn select_file(app_handle: &AppHandle) -> Option<Vec<PathBuf>> {
    let (tx, rx) = oneshot::channel();

    app_handle
        .dialog()
        .file()
        .set_title("Select a file to open")
        .add_filter("WAV files", &["wav"])
        .pick_files(move |paths| {
            let _ = tx.send(paths.map(|vec| {
                vec.into_iter()
                    .map(|p| p.into_path().unwrap_or_default())
                    .collect::<Vec<_>>()
            }));
        });

    rx.await.ok().flatten()
}

fn add_file_track(state: tauri::State<states::StateMixer>, path: PathBuf) {
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

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use crate::{project, track};

#[tauri::command]
pub fn get_track_list(app_handle: AppHandle) -> Result<track::track_list::TrackListResponse, ()> {
    let state_mixer_guard = app_handle.state::<project::states::StateMixerGuard>();

    let result = if let Ok(state_mixer) = state_mixer_guard.0.lock() {
        if let Ok(master_out) = state_mixer.master_out.lock() {
            if let Ok(list) = state_mixer.track_list.lock() {
                let mut response = list.as_response();
                response.tracks.insert(0, master_out.as_response());
                Ok(response)
            } else {
                app_handle
                    .dialog()
                    .message("Failed to lock track list")
                    .title("Track List Error")
                    .kind(MessageDialogKind::Warning)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
                Err(())
            }
        } else {
            app_handle
                .dialog()
                .message("Failed to lock master out")
                .title("Track List Error")
                .kind(MessageDialogKind::Warning)
                .buttons(MessageDialogButtons::Ok)
                .blocking_show();
            Err(())
        }
    } else {
        app_handle
            .dialog()
            .message("Failed to lock state mixer")
            .title("Track List Error")
            .kind(MessageDialogKind::Warning)
            .buttons(MessageDialogButtons::Ok)
            .blocking_show();
        Err(())
    };
    result
}

#[tauri::command]
pub fn update_track(
    app_handle: AppHandle,
    track_name: String,
    update: track::track_list::TrackUpdate,
) {
    let state_mixer_guard = app_handle.state::<project::states::StateMixerGuard>();
    if let Ok(state_mixer) = state_mixer_guard.0.lock() {
        if track_name == "master-out" {
            if let Ok(mut output) = state_mixer.master_out.lock() {
                match update {
                    track::track_list::TrackUpdate::Pan(pan) => output.pan = pan,
                    track::track_list::TrackUpdate::Gain(gain) => output.gain = gain,
                    _ => (),
                }
            } else {
                app_handle
                    .dialog()
                    .message("Failed to lock master out")
                    .title("Master Out Error")
                    .kind(MessageDialogKind::Warning)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
            }
        } else {
            if let Ok(mut list) = state_mixer.track_list.lock() {
                list.update_track(&track_name, update);
            } else {
                app_handle
                    .dialog()
                    .message("Failed to lock track list")
                    .title("Track List Error")
                    .kind(MessageDialogKind::Warning)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
            }
        }
    };
}

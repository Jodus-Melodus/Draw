use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use crate::{project, track};

#[tauri::command]
pub fn add_empty_track(app_handle: AppHandle) -> Result<(), ()> {
    let state_mixer_guard = app_handle.state::<project::states::StateMixerGuard>();
    let audio_context = app_handle.state::<project::states::StateAudioContext>();

    let res = if let Ok(state_mixer) = state_mixer_guard.0.lock() {
        if let Ok(mut track_list) = state_mixer.track_list.lock() {
            if let Some(input_device) = audio_context.input_device() {
                let number = track_list.track_list().len() + 1;
                let name = format!("track-{}", number);
                let source =
                    track::sources::source::StreamSource::new(input_device, app_handle.clone());
                let track = track::tracks::InputTrack::new(&name, Box::new(source));
                let new_track_name = format!("track-{}", number);
                track_list.add_track(&new_track_name, track);
                Ok(())
            } else {
                app_handle
                    .dialog()
                    .message("Invalid input device")
                    .title("Input Device Error")
                    .kind(MessageDialogKind::Warning)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
                Err(())
            }
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
            .message("Failed to lock state mixer")
            .title("State Error")
            .kind(MessageDialogKind::Warning)
            .buttons(MessageDialogButtons::Ok)
            .blocking_show();
        Err(())
    };
    res
}

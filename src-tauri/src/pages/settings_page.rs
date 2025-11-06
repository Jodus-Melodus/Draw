use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

pub fn open_settings(app: &AppHandle) {
    let settings_window = app.get_webview_window("settings");
    if let Some(window) = settings_window {
        let _ = window.show();
    } else {
        app.dialog()
            .message("Invalid input device")
            .title("Input Device Error")
            .kind(MessageDialogKind::Warning)
            .buttons(MessageDialogButtons::Ok)
            .blocking_show();
    }
}
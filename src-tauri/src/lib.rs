use tauri::{Emitter, Listener, Manager};

use crate::menu::handle_menu_events;

mod file;
mod menu;
mod settings;
mod states;
mod track;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let state_audio_context = states::StateAudioContext::new();
    let master_output_device = state_audio_context
        .output_device()
        .expect("Failed to get master output device");
    let state_mixer = states::StateMixer::new(master_output_device.clone());

    tauri::Builder::default()
        .manage(state_audio_context)
        .manage(state_mixer.clone())
        .setup(move |app| {
            let menu = menu::build_menus(app);
            app.set_menu(menu)?;

            let main_window = app.get_webview_window("main").unwrap();
            let mixer = state_mixer.clone();
            let main_window_clone = main_window.clone();
            main_window.listen("get-track-list", move |_| {
                let list = mixer.track_list.lock().unwrap();
                let response = list.as_response();

                main_window_clone
                    .emit("track-list-response", response)
                    .unwrap();
            });
            Ok(())
        })
        .on_menu_event(|app, event| {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                handle_menu_events(&app, &event).await;
            });
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

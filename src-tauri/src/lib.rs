use std::sync::{Arc, Mutex};

use tauri::Manager;

mod menus;
mod pages;
mod project;
mod track;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let state_audio_context = project::states::StateAudioContext::new();
    let master_output_device = state_audio_context
        .output_device()
        .expect("Failed to get master output device");

    tauri::Builder::default()
        .manage(state_audio_context)
        .setup(move |app| {
            let menu = menus::menu_builders::build_menus(app);
            app.set_menu(menu)?;

            let state_mixer = project::states::StateMixer::new(&app.handle(), master_output_device.clone());
            let state_mixer_guard = project::states::StateMixerGuard(Arc::new(Mutex::new(state_mixer)));
            app.handle().manage(state_mixer_guard);

            Ok(())
        })
        .on_menu_event(|app, event| {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                menus::menu_builders::handle_menu_events(&app, &event).await;
            });
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler!(
            track::commands::get_track_list,
            track::commands::update_track,
            menus::commands::add_empty_track
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

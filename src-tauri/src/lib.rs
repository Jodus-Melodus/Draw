use std::sync::{Arc, Mutex};

use tauri::Manager;

use crate::menus::menu_builders;

mod file;
mod menus;
mod pages;
mod project;
mod states;
mod track;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let state_audio_context = states::StateAudioContext::new();
    let master_output_device = state_audio_context
        .output_device()
        .expect("Failed to get master output device");

    tauri::Builder::default()
        .manage(state_audio_context)
        .setup(move |app| {
            // build and set menu
            let menu = menu_builders::build_menus(app);
            app.set_menu(menu)?;

            // create the StateMixer now that we have an `app` available
            let state_mixer = states::StateMixer::new(&app.handle(), master_output_device.clone());
            let state_mixer_guard = states::StateMixerGuard(Arc::new(Mutex::new(state_mixer)));
            app.handle().manage(state_mixer_guard);

            Ok(())
        })
        .on_menu_event(|app, event| {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                menu_builders::handle_menu_events(&app, &event).await;
            });
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            track::get_track_list,
            track::update_track,
            menus::project_menu::add_empty_track
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

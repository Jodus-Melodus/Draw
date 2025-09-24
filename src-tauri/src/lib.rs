use crate::menus::menu_builders;

mod file;
mod menus;
mod pages;
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
        .manage(state_mixer)
        .setup(|app| {
            let menu = menu_builders::build_menus(app);
            app.set_menu(menu)?;
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
            menus::project_menu::select_input_stream,
            menus::project_menu::add_track_stream,
            states::get_input_stream_device_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use crate::{
    menu::handle_menu_events,
    states::{StateAudioContext, StateAudioRecording, StateMixer}, track::add_track,
};

mod audio_input;
mod audio_output;
mod menu;
mod states;
mod types;
mod track;
mod settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state_audio_context = StateAudioContext::new(cpal::default_host().id());
    let state_audio_recording = StateAudioRecording::new();
    let master_output_device = state_audio_context.output_device().expect("Failed to get master output device");
    let state_mixer = StateMixer::new(master_output_device.clone());

    tauri::Builder::default()
        .manage(state_audio_context)
        .manage(state_audio_recording)
        .manage(state_mixer)
        .setup(|app| {
            let menu = menu::build_menus(app);
            app.set_menu(menu)?;
            Ok(())
        })
        .on_menu_event(|app, event| handle_menu_events(app, &event))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_track])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

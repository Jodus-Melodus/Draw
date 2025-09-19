use crate::{
    menu::handle_menu_events,
    states::{StateAudioContext, StateAudioRecording, StateMixer},
};

mod audio_input;
mod audio_output;
mod menu;
mod states;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state_audio_context = StateAudioContext::new(cpal::default_host().id());
    let state_audio_recording = StateAudioRecording::new();
    let state_mixer = StateMixer::new();

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

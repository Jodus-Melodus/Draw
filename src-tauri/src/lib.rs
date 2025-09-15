use std::sync::{atomic::AtomicBool, Arc};

use crate::{menu::handle_menu_events, types::RecordingState};

mod audio_input;
mod menu;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let recording = RecordingState {
        running: Arc::new(AtomicBool::new(false)),
    };

    tauri::Builder::default()
        .manage(recording)
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

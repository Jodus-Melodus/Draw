use crate::{menu::handle_menu_events, states::build_audio_context};

mod audio_input;
mod menu;
mod states;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_context = build_audio_context(cpal::HostId::Wasapi);

    tauri::Builder::default()
        .manage(audio_context)
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

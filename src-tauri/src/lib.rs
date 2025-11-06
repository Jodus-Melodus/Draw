use std::sync::{Arc, Mutex};

use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};
use tauri::{async_runtime, generate_context, generate_handler, Builder, Manager, WindowEvent};

mod menus;
mod pages;
mod project;
mod track;
mod types;

#[cfg_attr(mobile, mobile_entry_point)]
pub async fn run() {
    // TODO move to state
    let discord_rich_presence_client = Mutex::new(DiscordIpcClient::new("1435880809767637164"));

    if let Ok(mut client) = discord_rich_presence_client.lock() {
        client.connect().expect("Failed to connect to Discord");
        client
            .set_activity(
                Activity::new()
                    .state("DRAW")
                    .details("Making noise with DRAW"),
            )
            .expect("Failed to set activity");
    }

    let state_audio_context = project::states::StateAudioContext::new();
    let master_output_device = state_audio_context
        .output_device()
        .expect("Failed to get master output device");
    let state_mixer = project::states::StateMixer::new(master_output_device);
    let state_mixer_guard = project::states::StateMixerGuard(Arc::new(Mutex::new(state_mixer)));

    Builder::default()
        .manage(state_audio_context)
        .manage(state_mixer_guard)
        .setup(move |app| {
            let menu = menus::menu_builders::build_menus(app);
            app.set_menu(menu)?;
            Ok(())
        })
        .on_menu_event(|app, event| {
            let app = app.clone();
            async_runtime::spawn(async move {
                menus::menu_builders::handle_menu_events(&app, &event).await;
            });
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(generate_handler!(
            track::commands::get_track_list,
            track::commands::update_track,
            menus::commands::add_empty_track,
            project::commands::start_stream,
            project::commands::stop_stream,
            project::commands::load_project,
            project::commands::save_project,
        ))
        .on_window_event(move|w, e| match e {
            WindowEvent::CloseRequested { .. } => {
                if let Ok(mut client) = discord_rich_presence_client.lock() {
                    client.close().expect("Failed to close Discord connection");
                }

                if w.label() == "main" {
                    let app_handle = w.app_handle();
                    if let Some(win) = app_handle.get_webview_window("settings") {
                        let _ = win.close();
                    }
                    if let Some(win) = app_handle.get_webview_window("main") {
                        let _ = win.close();
                    }
                    app_handle.exit(0);
                } else {
                    let _ = w.close();
                }
            }
            _ => (),
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

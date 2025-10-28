use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use tauri::{
    menu::{
        CheckMenuItemBuilder, Menu, MenuBuilder, MenuEvent, MenuItemBuilder, MenuItemKind, Submenu,
        SubmenuBuilder,
    },
    App, AppHandle, Emitter, Manager, Wry,
};

use crate::{menus, pages, project, track};

fn build_file_menu(app: &App<Wry>) -> Submenu<Wry> {
    let open_file = MenuItemBuilder::new("Open file")
        .id("file-open-file")
        .accelerator("CmdOrCtrl+O")
        .build(app)
        .unwrap();

    let file_menu = SubmenuBuilder::new(app, "File")
        .id("file")
        .items(&[&open_file])
        .quit()
        .build()
        .unwrap();

    file_menu
}

fn build_preferences_menu(app: &App<Wry>) -> Submenu<Wry> {
    let audio_context = app.state::<project::states::StateAudioContext>();
    let output_device_registry = audio_context.output_device_registry.clone();
    let input_device_registry = audio_context.input_device_registry.clone();

    let output_menu = SubmenuBuilder::new(app, "Output Device")
        .id("preferences-output-devices")
        .build()
        .unwrap();

    for (i, output_device_name) in output_device_registry.list().iter().enumerate() {
        let id = format!("preferences-output-device-{}", i);
        let output_device = CheckMenuItemBuilder::new(output_device_name)
            .id(id)
            .checked(i == 0)
            .build(app)
            .unwrap();

        output_menu
            .append(&output_device)
            .expect("Failed to add item to menu");
    }

    let input_menu = SubmenuBuilder::new(app, "Input Device")
        .id("preferences-input-devices")
        .build()
        .unwrap();

    for (i, input_device_name) in input_device_registry.list().iter().enumerate() {
        let id = format!("preferences-input-device-{}", i);
        let input_device = CheckMenuItemBuilder::new(input_device_name)
            .id(id)
            .checked(i == 0)
            .build(app)
            .unwrap();

        input_menu
            .append(&input_device)
            .expect("Failed to add item to menu");
    }

    let settings = MenuItemBuilder::new("Settings")
        .id("preferences-settings")
        .accelerator("CmdOrCtrl+,")
        .build(app)
        .unwrap();

    let preferences_menu = SubmenuBuilder::new(app, "Preferences")
        .id("preferences")
        .items(&[&output_menu, &input_menu, &settings])
        .build()
        .unwrap();

    preferences_menu
}

fn build_project_menu(app: &App<Wry>) -> Submenu<Wry> {
    let add_track = MenuItemBuilder::new("Add Track")
        .accelerator("CmdOrCtrl+A")
        .id("project-add-track")
        .build(app)
        .unwrap();

    let save_project = MenuItemBuilder::new("Save project")
        .accelerator("CmdOrCtrl+S")
        .id("project-save-project")
        .build(app)
        .unwrap();

    let load_project = MenuItemBuilder::new("Open project")
        .accelerator("CmdOrCtrl+L")
        .id("project-open-project")
        .build(app)
        .unwrap();

    let project_menu = SubmenuBuilder::new(app, "Project")
        .items(&[&add_track, &save_project, &load_project])
        .build()
        .unwrap();
    project_menu
}

pub fn build_menus(app: &App<Wry>) -> Menu<Wry> {
    let file_menu = build_file_menu(app);
    let project_menu = build_project_menu(app);
    let preferences_menu = build_preferences_menu(app);
    MenuBuilder::new(app)
        .items(&[&file_menu, &project_menu, &preferences_menu])
        .build()
        .unwrap()
}

pub async fn handle_menu_events(app_handle: &AppHandle, event: &MenuEvent) {
    let audio_context = app_handle.state::<project::states::StateAudioContext>();
    let id: &str = event.id.0.as_ref();

    match id {
        "file-open-file" => project::file::open_files(app_handle).await,
        "preferences-settings" => pages::settings_page::open_settings(app_handle),
        "project-add-track" => {
            menus::commands::add_empty_track(app_handle.clone()).unwrap();
            let window = app_handle
                .get_webview_window("main")
                .expect("Failed to get main window");
            window.emit("updated-track-list", ()).unwrap();
        }
        "project-save-project" => project::commands::save_project(app_handle.clone()),
        "project-open-project" => project::commands::load_project(app_handle.clone()),
        _ if id.starts_with("preferences-output-device-") => {
            update_master_io_device_index(audio_context.output_device_index.clone(), id);
            update_radio_group_menu(app_handle, id);
            update_master_output_device_track(app_handle);
        }
        _ if id.starts_with("preferences-input-device-") => {
            update_master_io_device_index(audio_context.input_device_index.clone(), id);
            update_radio_group_menu(app_handle, id);
        }
        _ => eprintln!("Unknown menu item selected"),
    }
}

fn update_master_io_device_index(device_index: Arc<AtomicUsize>, id: &str) {
    let parts = id.split("-").collect::<Vec<_>>();
    let index = parts
        .last()
        .expect("Failed to get indices")
        .parse::<usize>()
        .expect("Failed to convert to index");
    device_index.store(index, Ordering::SeqCst);
}

fn update_master_output_device_track(app: &AppHandle) {
    let state_mixer_guard = app.state::<project::states::StateMixerGuard>();
    let state_mixer = state_mixer_guard.0.lock().unwrap();
    let out = state_mixer.master_out.clone();
    let mut master_output = out.lock().unwrap();
    let audio_context = app.state::<project::states::StateAudioContext>();
    let new_master_output_device = audio_context
        .output_device()
        .expect("Failed to get new master output device");
    let new_output_source = track::sources::sink::StreamSink::new(
        new_master_output_device,
        state_mixer.track_list.clone(),
        out.clone(),
    );
    master_output.sink = Box::new(new_output_source);
}

fn update_radio_group_menu(app: &AppHandle, id: &str) {
    if let Some(menu) = app.menu() {
        for main_menu in menu.items().expect("Failed to get menus") {
            loop_through_sub_menus(id, main_menu);
        }
    }
}

fn loop_through_sub_menus(id: &str, main_menu: MenuItemKind<Wry>) {
    if main_menu.id().0 == "preferences" {
        if let MenuItemKind::Submenu(device_menu) = main_menu {
            for menu_item in device_menu.items().expect("Failed to get sub menu items") {
                if (menu_item.id().0 == "preferences-output-devices"
                    && id.starts_with("preferences-output"))
                    || (menu_item.id().0 == "preferences-input-devices"
                        && id.starts_with("preferences-input"))
                {
                    uncheck_other_devices(id, menu_item);
                }
            }
        }
    }
}

fn uncheck_other_devices(id: &str, menu_item: MenuItemKind<Wry>) {
    if let MenuItemKind::Submenu(io_devices) = menu_item {
        for io_device in io_devices.items().expect("Failed to get menus") {
            if let MenuItemKind::Check(device) = io_device {
                device
                    .set_checked(device.id().0 == id)
                    .expect("Failed to uncheck item");
            }
        }
    }
}

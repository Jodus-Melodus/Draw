use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use tauri::{
    menu::{
        CheckMenuItemBuilder, Menu, MenuBuilder, MenuEvent, MenuItemBuilder, MenuItemKind, Submenu,
        SubmenuBuilder,
    },
    App, AppHandle, Manager, Wry,
};

use crate::{
    menus::project_menu::{add_track_file, add_track_stream}, pages::settings_page::open_settings, states
};

fn build_file_menu(app: &App<Wry>) -> Submenu<Wry> {
    let settings = MenuItemBuilder::new("Settings")
        .id("file-settings")
        .accelerator("CmdOrCtrl+,")
        .build(app)
        .unwrap();

    let file_menu = SubmenuBuilder::new(app, "File")
        .id("file")
        .item(&settings)
        .quit()
        .build()
        .unwrap();

    file_menu
}

fn build_project_menu(app: &App<Wry>) -> Submenu<Wry> {
    let add_track_file = MenuItemBuilder::new("File Track")
        .id("project-add-track-file")
        .accelerator("CmdOrCtrl+F")
        .build(app)
        .unwrap();

    let add_track_stream = MenuItemBuilder::new("Stream Track")
        .id("project-add-track-stream")
        .accelerator("CmdOrCtrl+S")
        .build(app)
        .unwrap();

    let add_track = SubmenuBuilder::new(app, "Add Track")
        .id("project-add-track")
        .items(&[&add_track_file, &add_track_stream])
        .build()
        .unwrap();

    let project_menu = SubmenuBuilder::new(app, "Project")
        .items(&[&add_track])
        .build()
        .unwrap();
    project_menu
}

fn build_device_menu(app: &App<Wry>) -> Submenu<Wry> {
    let audio_context = app.state::<states::StateAudioContext>();
    let input_device_registry = audio_context.input_device_registry.clone();
    let output_device_registry = audio_context.output_device_registry.clone();

    let input_menu = SubmenuBuilder::new(app, "Input Device")
        .id("devices-input-devices")
        .build()
        .unwrap();

    for (i, input_device_name) in input_device_registry.list().iter().enumerate() {
        let id = format!("devices-input-{}", i);
        let input_device = CheckMenuItemBuilder::new(input_device_name)
            .id(id)
            .checked(i == 0)
            .build(app)
            .unwrap();

        input_menu
            .append(&input_device)
            .expect("Failed to add item to menu");
    }

    let output_menu = SubmenuBuilder::new(app, "Output Device")
        .id("devices-output-devices")
        .build()
        .unwrap();

    for (i, output_device_name) in output_device_registry.list().iter().enumerate() {
        let id = format!("devices-output-{}", i);
        let output_device = CheckMenuItemBuilder::new(output_device_name)
            .id(id)
            .checked(i == 0)
            .build(app)
            .unwrap();

        output_menu
            .append(&output_device)
            .expect("Failed to add item to menu");
    }

    let device_menu = SubmenuBuilder::new(app, "Devices")
        .id("devices")
        .item(&input_menu)
        .item(&output_menu)
        .build()
        .unwrap();

    device_menu
}

pub fn build_menus(app: &App<Wry>) -> Menu<Wry> {
    let file_menu = build_file_menu(app);
    let project_menu = build_project_menu(app);
    let device_menu = build_device_menu(app);
    MenuBuilder::new(app)
        .items(&[&file_menu, &project_menu, &device_menu])
        .build()
        .unwrap()
}

pub async fn handle_menu_events(app: &AppHandle, event: &MenuEvent) {
    let audio_context = app.state::<states::StateAudioContext>();
    let _mixer_state = app.state::<states::StateMixer>();
    let id: &str = event.id.0.as_ref();

    match id {
        "file-settings" => open_settings(app),
        "project-add-track-file" => add_track_file(app).await,
        "project-add-track-stream" => add_track_stream(app),
        _ => {
            if id.starts_with("devices-input") {
                update_device_index(audio_context.input_device_index.clone(), id);
                update_radio_group_menu(app, id);
            } else if id.starts_with("devices-output") {
                update_device_index(audio_context.output_device_index.clone(), id);
                update_radio_group_menu(app, id);
            } else {
                eprintln!("Unknown menu item selected");
            }
        }
    }
}

fn update_device_index(device_index: Arc<AtomicUsize>, id: &str) {
    let parts = id.split("-").collect::<Vec<_>>();
    let index = parts
        .last()
        .expect("Failed to get indices")
        .parse::<usize>()
        .expect("Failed to convert to index");
    device_index.store(index, Ordering::SeqCst);
}

fn update_radio_group_menu(app: &AppHandle, id: &str) {
    if let Some(menu) = app.menu() {
        for main_menu in menu.items().expect("Failed to get menus") {
            loop_through_sub_menus(id, main_menu);
        }
    }
}

fn loop_through_sub_menus(id: &str, main_menu: MenuItemKind<Wry>) {
    if main_menu.id().0 == "devices" {
        if let MenuItemKind::Submenu(device_menu) = main_menu {
            for menu_item in device_menu.items().expect("Failed to get sub menu items") {
                if menu_item.id().0 == "devices-input-devices" && id.starts_with("devices-input") {
                    uncheck_other_devices(id, menu_item);
                } else if menu_item.id().0 == "devices-output-devices"
                    && id.starts_with("devices-output")
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

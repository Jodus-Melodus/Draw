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
    audio_input::{graph_recording, save_file, start_audio_input, stop_audio_input},
    types::AudioContext,
};

fn build_file_menu(app: &App<Wry>) -> Submenu<Wry> {
    let open_file = MenuItemBuilder::new("Open File")
        .id("file-open-file")
        .accelerator("CmdOrCtrl+O")
        .build(app)
        .unwrap();

    let save_file = MenuItemBuilder::new("Save")
        .id("file-save-file")
        .accelerator("CmdOrCtrl+S")
        .build(app)
        .unwrap();

    let save_as_file = MenuItemBuilder::new("Save As")
        .id("file-save-as-file")
        .accelerator("CmdOrCtrl+Shift+S")
        .build(app)
        .unwrap();

    let settings = MenuItemBuilder::new("Settings")
        .id("file-settings")
        .accelerator("CmdOrCtrl+,")
        .build(app)
        .unwrap();

    let start_record = MenuItemBuilder::new("Start Record")
        .id("file-start-record")
        .build(app)
        .unwrap();

    let stop_record = MenuItemBuilder::new("Stop Record")
        .id("file-stop-record")
        .build(app)
        .unwrap();

    let graph_recording = MenuItemBuilder::new("Graph Builder")
        .id("graph-builder")
        .build(app)
        .unwrap();

    let save = MenuItemBuilder::new("Save").id("save").build(app).unwrap();

    let file_menu = SubmenuBuilder::new(app, "File")
        .id("file")
        .item(&open_file)
        .item(&save_file)
        .item(&save_as_file)
        .separator()
        .item(&settings)
        .item(&start_record)
        .item(&stop_record)
        .item(&graph_recording)
        .item(&save)
        .quit()
        .build()
        .unwrap();

    file_menu
}

fn build_device_menu(app: &App<Wry>) -> Submenu<Wry> {
    let state: tauri::State<'_, AudioContext> = app.state();
    let input_device_registry = state.input_device_registry.clone();
    let output_device_registry = state.output_device_registry.clone();

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
    let device_menu = build_device_menu(app);
    MenuBuilder::new(app)
        .items(&[&file_menu, &device_menu])
        .build()
        .unwrap()
}

pub fn handle_menu_events(app: &AppHandle, event: &MenuEvent) {
    let state = app.state();
    let id: &str = event.id.0.as_ref();

    match id {
        "file-open-file" => eprintln!("Not yet implemented"), // TODO
        "file-save-file" => eprintln!("Not yet implemented"), // TODO
        "file-save-as-file" => eprintln!("Not yet implemented"), // TODO
        "file-settings" => eprintln!("Not yet implemented"),  // TODO
        "file-start-record" => start_audio_input(state),
        "file-stop-record" => stop_audio_input(state),
        "graph-builder" => graph_recording(state),
        "save" => save_file(state),
        _ => {
            if id.starts_with("devices-input") {
                update_device_index(state.input_device_index.clone(), id);
                update_radio_group_menu(app, id);
            } else if id.starts_with("devices-output") {
                update_device_index(state.output_device_index.clone(), id);
                update_radio_group_menu(app, id);
            } else {
                eprintln!("Unknown menu item selected"); // :|
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

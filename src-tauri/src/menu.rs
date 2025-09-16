use tauri::{
    menu::{Menu, MenuBuilder, MenuEvent, MenuItemBuilder, Submenu, SubmenuBuilder},
    App, AppHandle, Manager, Wry,
};

use crate::{
    audio_input::{start_audio_input, stop_audio_input},
    types::AudioContext,
};

fn build_file_menu(app: &App<Wry>) -> Submenu<Wry> {
    let open_file = MenuItemBuilder::new("Open File")
        .id("open-file")
        .accelerator("CmdOrCtrl+O")
        .build(app)
        .unwrap();

    let save_file = MenuItemBuilder::new("Save")
        .id("save-file")
        .accelerator("CmdOrCtrl+S")
        .build(app)
        .unwrap();

    let save_as_file = MenuItemBuilder::new("Save As")
        .id("save-as-file")
        .accelerator("CmdOrCtrl+Shift+S")
        .build(app)
        .unwrap();

    let settings = MenuItemBuilder::new("Settings")
        .id("settings")
        .accelerator("CmdOrCtrl+,")
        .build(app)
        .unwrap();

    let start_record = MenuItemBuilder::new("Start Record")
        .id("start-record")
        .build(app)
        .unwrap();

    let stop_record = MenuItemBuilder::new("Stop Record")
        .id("stop-record")
        .build(app)
        .unwrap();

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&open_file)
        .item(&save_file)
        .item(&save_as_file)
        .separator()
        .item(&settings)
        .item(&start_record)
        .item(&stop_record)
        .quit()
        .build()
        .unwrap();

    file_menu
}

fn build_device_menu(app: &App<Wry>) -> Submenu<Wry> {
    let state: tauri::State<'_, AudioContext> = app.state();
    let input_device_registry = state.input_device_registry.clone();
    let output_device_registry = state.output_device_registry.clone();

    let input_menu = SubmenuBuilder::new(app, "Input Device").build().unwrap();

    for input_device_name in input_device_registry.list() {
        let id = input_device_name.replace(" ", "-");
        let input_device = MenuItemBuilder::new(input_device_name)
            .id(id)
            .build(app)
            .unwrap();
        input_menu
            .append(&input_device)
            .expect("Failed to add item to menu");
    }

    let output_menu = SubmenuBuilder::new(app, "Output Device").build().unwrap();

    for output_device_name in output_device_registry.list() {
        let id = output_device_name.replace(" ", "-");
        let output_device = MenuItemBuilder::new(output_device_name)
            .id(id)
            .build(app)
            .unwrap();
        output_menu
            .append(&output_device)
            .expect("Failed to add item to menu");
    }

    let device_menu = SubmenuBuilder::new(app, "Devices")
        .item(&input_menu)
        .item(&output_menu)
        .build()
        .unwrap();

    device_menu
}

pub fn handle_menu_events(app: &AppHandle, event: &MenuEvent) {
    let state = app.state();

    match event.id().0.as_str() {
        "open-file" => todo!(),
        "save-file" => todo!(),
        "save-as-file" => todo!(),
        "settings" => todo!(),
        "start-record" => start_audio_input(state),
        "stop-record" => stop_audio_input(state),
        _ => {}
    }
}

pub fn build_menus(app: &App<Wry>) -> Menu<Wry> {
    let file_menu = build_file_menu(app);
    let device_menu = build_device_menu(app);
    MenuBuilder::new(app)
        .items(&[&file_menu, &device_menu])
        .build()
        .unwrap()
}

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

use crate::{file, menus, pages, states, track};

fn build_file_menu(app: &App<Wry>) -> Submenu<Wry> {
    let open_file = MenuItemBuilder::new("Open file")
        .id("file-open-file")
        .accelerator("CmdOrCtrl+O")
        .build(app)
        .unwrap();

    let settings = MenuItemBuilder::new("Settings")
        .id("file-settings")
        .accelerator("CmdOrCtrl+,")
        .build(app)
        .unwrap();

    let audio_context = app.state::<states::StateAudioContext>();
    let output_device_registry = audio_context.output_device_registry.clone();

    let output_menu = SubmenuBuilder::new(app, "Output Device")
        .id("file-output-devices")
        .build()
        .unwrap();

    for (i, output_device_name) in output_device_registry.list().iter().enumerate() {
        let id = format!("file-output-device-{}", i);
        let output_device = CheckMenuItemBuilder::new(output_device_name)
            .id(id)
            .checked(i == 0)
            .build(app)
            .unwrap();

        output_menu
            .append(&output_device)
            .expect("Failed to add item to menu");
    }

    let file_menu = SubmenuBuilder::new(app, "File")
        .id("file")
        .items(&[&open_file])
        .separator()
        .items(&[&settings, &output_menu])
        .quit()
        .build()
        .unwrap();

    file_menu
}

fn build_project_menu(app: &App<Wry>) -> Submenu<Wry> {
    let add_track = MenuItemBuilder::new("Add Track")
        .accelerator("CmdOrCtrl+A")
        .id("project-add-track")
        .build(app)
        .unwrap();

    let project_menu = SubmenuBuilder::new(app, "Project")
        .items(&[&add_track])
        .build()
        .unwrap();
    project_menu
}

pub fn build_menus(app: &App<Wry>) -> Menu<Wry> {
    let file_menu = build_file_menu(app);
    let project_menu = build_project_menu(app);
    MenuBuilder::new(app)
        .items(&[&file_menu, &project_menu])
        .build()
        .unwrap()
}

pub async fn handle_menu_events(app: &AppHandle, event: &MenuEvent) {
    let audio_context = app.state::<states::StateAudioContext>();
    let mixer_state = app.state::<states::StateMixer>();
    let id: &str = event.id.0.as_ref();

    match id {
        "file-open-file" => file::open_file(app).await,
        "file-settings" => pages::settings_page::open_settings(app),
        "project-add-track" => menus::project_menu::add_empty_track(mixer_state.clone()),
        _ if id.starts_with("file-output-device-") => {
            update_master_output_device_index(audio_context.output_device_index.clone(), id);
            update_radio_group_menu(app, id);
            update_master_output_device_track(app);
        }
        _ => eprintln!("Unknown menu item selected"),
    }
}

fn update_master_output_device_index(device_index: Arc<AtomicUsize>, id: &str) {
    let parts = id.split("-").collect::<Vec<_>>();
    let index = parts
        .last()
        .expect("Failed to get indices")
        .parse::<usize>()
        .expect("Failed to convert to index");
    device_index.store(index, Ordering::SeqCst);
}

fn update_master_output_device_track(app: &AppHandle) {
    let mixer = app.state::<states::StateMixer>();
    let list = mixer.track_list.clone();
    let track_list = list.lock().expect("Failed to lock list");
    let master_output_track = track_list
        .get_track("master-out")
        .expect("Failed to get master output track")
        .clone();
    let mut master_output = master_output_track
        .lock()
        .expect("Failed to lock master output");
    let audio_context = app.state::<states::StateAudioContext>();
    let new_master_output_device = audio_context
        .output_device()
        .expect("Failed to get new master output device");
    let new_output_source = track::StreamSource::new(new_master_output_device.clone());
    master_output.stream_source = Some(new_output_source);
}

fn update_radio_group_menu(app: &AppHandle, id: &str) {
    if let Some(menu) = app.menu() {
        for main_menu in menu.items().expect("Failed to get menus") {
            loop_through_sub_menus(id, main_menu);
        }
    }
}

fn loop_through_sub_menus(id: &str, main_menu: MenuItemKind<Wry>) {
    if main_menu.id().0 == "file" {
        if let MenuItemKind::Submenu(device_menu) = main_menu {
            for menu_item in device_menu.items().expect("Failed to get sub menu items") {
                if menu_item.id().0 == "file-output-devices" && id.starts_with("file-output") {
                    uncheck_other_devices(id, menu_item);
                }
            }
        }
    }
}

fn uncheck_other_devices(id: &str, menu_item: MenuItemKind<Wry>) {
    if let MenuItemKind::Submenu(o_devices) = menu_item {
        for o_device in o_devices.items().expect("Failed to get menus") {
            if let MenuItemKind::Check(device) = o_device {
                device
                    .set_checked(device.id().0 == id)
                    .expect("Failed to uncheck item");
            }
        }
    }
}

use tauri::{
    menu::{Menu, MenuBuilder, MenuItemBuilder, Submenu, SubmenuBuilder},
    App, Wry,
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

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&open_file)
        .item(&save_file)
        .item(&save_as_file)
        .separator()
        .item(&settings)
        .quit()
        .build()
        .unwrap();

    file_menu
}

// TODO handler for menu events

pub fn build_menus(app: &App<Wry>) -> Menu<Wry> {
    let file_menu = build_file_menu(app);
    MenuBuilder::new(app).items(&[&file_menu]).build().unwrap()
}

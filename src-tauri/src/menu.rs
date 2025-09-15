use tauri::{
    menu::{Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    App, Wry,
};

pub fn build_menu(app: &App<Wry>) -> Menu<Wry> {
    let open_file = MenuItemBuilder::new("Open File")
        .id("open-file")
        .accelerator("CmdOrCtrl+o")
        .build(app)
        .unwrap();

    let settings = MenuItemBuilder::new("Settings")
        .id("settings")
        .accelerator("CmdOrCtrl+,")
        .build(app)
        .unwrap();

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&open_file)
        .separator()
        .item(&settings)
        .quit()
        .build()
        .unwrap();

    MenuBuilder::new(app).items(&[&file_menu]).build().unwrap()
}

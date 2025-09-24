use tauri::AppHandle;

pub fn open_settings(app: &AppHandle) {
    tauri::WebviewWindowBuilder::new(
        app,
        "settings",
        tauri::WebviewUrl::App("settings.html".into()),
    )
    .title("Settings")
    .always_on_top(true)
    .center()
    .maximizable(false)
    .menu(tauri::menu::Menu::new(app).unwrap())
    .minimizable(false)
    .resizable(false)
    .inner_size(800.0, 400.0)
    .skip_taskbar(true)
    .build()
    .unwrap();
}

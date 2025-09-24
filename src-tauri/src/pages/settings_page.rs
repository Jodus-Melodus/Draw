use tauri::AppHandle;

#[tauri::command]
pub fn open_settings(app: &AppHandle) {
    tauri::WebviewWindowBuilder::new(
        app,
        "settings",
        tauri::WebviewUrl::App("settings.html".into()),
    )
    .title("Settings")
    .menu(tauri::menu::Menu::new(app).unwrap())
    .build()
    .unwrap();
}

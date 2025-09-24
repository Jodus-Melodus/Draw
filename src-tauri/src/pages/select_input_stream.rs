use tauri::AppHandle;

pub fn open_select_input_stream(app: &AppHandle) {
    tauri::WebviewWindowBuilder::new(
        app,
        "select-input-stream",
        tauri::WebviewUrl::App("inputStreams.html".into()),
    )
    .title("Select Input Stream")
    .always_on_top(true)
    .center()
    .maximizable(false)
    .menu(tauri::menu::Menu::new(app).unwrap())
    .minimizable(false)
    .resizable(false)
    .inner_size(400.0, 600.0)
    .skip_taskbar(true)
    .build()
    .unwrap();
}

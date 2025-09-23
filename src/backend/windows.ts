const { WebviewWindow, event } = (window as any).__TAURI__;
const { listen, emit } = event;

/**
 * Opens settings page
 */
function openSettings() {
    new WebviewWindow("settings", { url: "settings.html" });
}

listen("open-settings", () => openSettings());


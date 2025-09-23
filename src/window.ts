import { listen } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

/**
 * Opens settings page
 */
function openSettings() {
    new WebviewWindow("settings", { url: "settings.html" });
}

listen("open-settings", () => openSettings());


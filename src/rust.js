const { invoke } = window.__TAURI__.core;
const { WebviewWindow } = window.__TAURI__.window;
const { listen } = window.__TAURI__.event;

/**
 * Opens settings page
 */
function openSettings() {
    new WebviewWindow("settings", {url: "settings.html"});
}

listen("open-settings", (_event) => {
    openSettings()
})

/**
 * Adds a track to the track list
 */
export async function callRust() {
    try {
        const message = await invoke("add_track", {});
        const div = document.createElement("div");
        div.textContent = message;
        document.body.append(div);
    } catch (error) {
        console.log("Error calling Rust:", error);
    }
}

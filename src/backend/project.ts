import { invoke } from "@tauri-apps/api/core";

export async function startRecording() {
    try {
        await invoke("start_recording", {});
        console.log("Started recording");
    } catch (err) {
        console.error("Failed to start recording:", err);
    }
}

export async function stopRecording() {
    try {
        await invoke("stop_recording", {});
        console.log("Stopped recording");
    } catch (err) {
        console.error("Failed to stop recording:", err);
    }
}
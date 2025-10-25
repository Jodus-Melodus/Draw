import { invoke } from "@tauri-apps/api/core";

export async function startStream() {
    try {
        await invoke("start_stream", {});
        console.log("Started stream");
    } catch (err) {
        console.error("Failed to start stream:", err);
    }
}

export async function stopStream() {
    try {
        await invoke("stop_stream", {});
        console.log("Stopped stream");
    } catch (err) {
        console.error("Failed to stop stream:", err);
    }
}
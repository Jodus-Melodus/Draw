import { invoke } from "@tauri-apps/api/core";

export async function selectInputStreamDeviceIndex(deviceIndex: number) {
    await invoke("select_input_stream", { deviceIndex });
}

export async function selectInputStreamDevice() {
    await invoke("add_track_stream");
}
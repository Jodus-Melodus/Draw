import { invoke } from "@tauri-apps/api/core";

export async function selectInputStreamDeviceIndex(deviceIndex: number) {
    await invoke("select_input_stream", { deviceIndex });
}

export async function selectInputStreamDevice() {
    await invoke("add_track_stream");
}

export async function getInputStreamDeviceList(): Promise<string[]> {
    const list = await invoke<string[]>("get_input_stream_device_list");
    return list;
}
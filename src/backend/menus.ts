import { invoke } from "@tauri-apps/api/core";

/**
 * Update the index to select an input stream device
 * @param deviceIndex the index of the selected input device for the new stream track
 */
export async function selectInputStreamDeviceIndex(deviceIndex: number) {
    await invoke("select_input_stream", { deviceIndex });
}

/**
 * Add a new input device with the selected stream
 * Call `selectInputDeviceIndex` before you call this
 */
export async function selectInputStreamDevice() {
    await invoke("add_track_stream");
}

/**
 * Get the names of the available input streams
 * @returns a list of input stream device names
 */
export async function getInputStreamDeviceList(): Promise<string[]> {
    const list = await invoke<string[]>("get_input_stream_device_list");
    return list;
}
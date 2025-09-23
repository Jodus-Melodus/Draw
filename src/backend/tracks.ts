import type { TrackListResponse } from "./types.js";

const { WebviewWindow, event } = (window as any).__TAURI__;
const { listen, emit } = event;

/**
 * Request track list from Rust
 */
export function getTrackList(): Promise<TrackListResponse> {
    return new Promise((resolve) => {
        listen("track-list-response", (event: any) => {
            resolve(event.payload as TrackListResponse);
        });
        emit("get-track-list");
    });
}

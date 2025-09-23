import { emit, listen } from "@tauri-apps/api/event";
import type { TrackListResponse } from "./types.js";

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

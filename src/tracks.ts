import type { TrackListResponse, TrackUpdate } from "./types.js";
import { invoke } from "@tauri-apps/api/core";

/**
 * Get a list of all the input and output tracks
 * @returns the current track list
 */
export async function getTrackList(): Promise<TrackListResponse> {
    const trackList = await invoke<TrackListResponse>("get_track_list");
    return trackList;
}

/**
 * Update a track in the track list
 * @param trackName the name of the track  you want to update
 * @param update the update you want to make using the `TrackUpdate` type
 */
export async function updateTrack(trackName: string, update: TrackUpdate) {
    try {
        await invoke("update_track", {
            trackName,
            update,
        });
        console.log("Updated track successfully");
    } catch (err) {
        console.error("Failed to update track:", err);
    }
}
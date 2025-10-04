import type { TrackInfo, TrackListResponse, TrackUpdate } from "./types.js";
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
/**
 * Add an empty track to the track list
 */
export async function addEmptyTrack() {
    try {
        await invoke("add_empty_track");
    } catch (err) {
        console.error("Failed to add an empty track:", err);
    }
}

export function addNewTrack(trackTemplate: HTMLTemplateElement, channelTrackTemplate: HTMLTemplateElement, track: TrackInfo, trackContainer: HTMLElement, channelTrackContainer: HTMLElement) {
    const newTrack = trackTemplate.content.cloneNode(true) as DocumentFragment;
    const newChannel = channelTrackTemplate.content.cloneNode(true) as DocumentFragment;

    const trackName = newTrack.querySelector(".track-name") as HTMLElement;
    const trackMuteButton = newTrack.querySelector(".track-mute") as HTMLElement;
    const trackSoloButton = newTrack.querySelector(".track-solo") as HTMLElement;
    const trackRecordButton = newTrack.querySelector(".track-record") as HTMLElement;
    const trackMonitorButton = newTrack.querySelector(".track-monitor") as HTMLElement;

    const channelName = newChannel.querySelector(".channel-name") as HTMLElement;
    const channelMuteButton = newChannel.querySelector(".channel-mute") as HTMLElement;
    const channelSoloButton = newChannel.querySelector(".channel-solo") as HTMLElement;
    const channelRecordButton = newChannel.querySelector(".channel-record") as HTMLElement;
    const channelMonitorButton = newChannel.querySelector(".channel-monitor") as HTMLElement;

    trackName.textContent = track.name;
    channelName.textContent = track.name;

    trackMuteButton.addEventListener("click", () => {
        if (trackMuteButton.classList.contains("active")) {
            trackMuteButton.classList.remove("active");
            channelMuteButton.classList.remove("active");
        } else {
            trackMuteButton.classList.add("active");
            trackSoloButton.classList.remove("active");
            channelMuteButton.classList.add("active");
            channelSoloButton.classList.remove("active");
        }
    });

    trackSoloButton.addEventListener("click", () => {
        if (trackSoloButton.classList.contains("active")) {
            trackSoloButton.classList.remove("active");
            channelSoloButton.classList.remove("active");
        } else {
            trackSoloButton.classList.add("active");
            trackMuteButton.classList.remove("active");
            channelSoloButton.classList.add("active");
            channelMuteButton.classList.remove("active");
        }
    });

    trackRecordButton.addEventListener("click", () => {
        if (trackRecordButton.classList.contains("active")) {
            trackRecordButton.classList.remove("active");
            channelRecordButton.classList.remove("active");
        } else {
            trackRecordButton.classList.add("active");
            channelRecordButton.classList.add("active");
        }
    });

    trackMonitorButton.addEventListener("click", () => {
        if (trackMonitorButton.classList.contains("active")) {
            trackMonitorButton.classList.remove("active");
            channelMonitorButton.classList.remove("active");
        } else {
            trackMonitorButton.classList.add("active");
            channelMonitorButton.classList.add("active");
        }
    });

    channelMuteButton.addEventListener("click", () => {
        if (trackMuteButton.classList.contains("active")) {
            trackMuteButton.classList.remove("active");
            channelMuteButton.classList.remove("active");
        } else {
            trackMuteButton.classList.add("active");
            trackSoloButton.classList.remove("active");
            channelMuteButton.classList.add("active");
            channelSoloButton.classList.remove("active");
        }
    });

    channelSoloButton.addEventListener("click", () => {
        if (trackSoloButton.classList.contains("active")) {
            trackSoloButton.classList.remove("active");
            channelSoloButton.classList.remove("active");
        } else {
            trackSoloButton.classList.add("active");
            trackMuteButton.classList.remove("active");
            channelSoloButton.classList.add("active");
            channelMuteButton.classList.remove("active");
        }
    });

    channelRecordButton.addEventListener("click", () => {
        if (trackRecordButton.classList.contains("active")) {
            trackRecordButton.classList.remove("active");
            channelRecordButton.classList.remove("active");
        } else {
            trackRecordButton.classList.add("active");
            channelRecordButton.classList.add("active");
        }
    });

    channelMonitorButton.addEventListener("click", () => {
        if (trackMonitorButton.classList.contains("active")) {
            trackMonitorButton.classList.remove("active");
            channelMonitorButton.classList.remove("active");
        } else {
            trackMonitorButton.classList.add("active");
            channelMonitorButton.classList.add("active");
        }
    });

    trackContainer.appendChild(newTrack);
    channelTrackContainer.appendChild(newChannel);
}
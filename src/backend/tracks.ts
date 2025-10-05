import { updateTrackList } from "../main.js";
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

    let tracks = await getTrackList();
    tracks.tracks.forEach(track => {
        console.log(track);
    });

    updateTrackList();
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

    trackMuteButton.addEventListener("click", async () => {
        var active = trackMuteButton.classList.contains("active");
        if (active) {
            trackMuteButton.classList.remove("active");
            channelMuteButton.classList.remove("active");
        } else {
            trackMuteButton.classList.add("active");
            trackSoloButton.classList.remove("active");
            channelMuteButton.classList.add("active");
            channelSoloButton.classList.remove("active");
        }

        await updateTrack(track.name, { Mute: !active });
    });

    trackSoloButton.addEventListener("click", async () => {
        var active = trackSoloButton.classList.contains("active");
        if (active) {
            trackSoloButton.classList.remove("active");
            channelSoloButton.classList.remove("active");
        } else {
            trackSoloButton.classList.add("active");
            trackMuteButton.classList.remove("active");
            channelSoloButton.classList.add("active");
            channelMuteButton.classList.remove("active");
        }

        await updateTrack(track.name, { Solo: !active });
    });

    trackRecordButton.addEventListener("click", async () => {
        var active = trackRecordButton.classList.contains("active");
        if (active) {
            trackRecordButton.classList.remove("active");
            channelRecordButton.classList.remove("active");
        } else {
            trackRecordButton.classList.add("active");
            channelRecordButton.classList.add("active");
        }

        await updateTrack(track.name, { Record: !active });
    });

    trackMonitorButton.addEventListener("click", async () => {
        var active = trackMonitorButton.classList.contains("active");
        if (active) {
            trackMonitorButton.classList.remove("active");
            channelMonitorButton.classList.remove("active");
        } else {
            trackMonitorButton.classList.add("active");
            channelMonitorButton.classList.add("active");
        }

        await updateTrack(track.name, { Monitor: !active });
    });

    channelMuteButton.addEventListener("click", async () => {
        var active = trackMuteButton.classList.contains("active");
        if (active) {
            trackMuteButton.classList.remove("active");
            channelMuteButton.classList.remove("active");
        } else {
            trackMuteButton.classList.add("active");
            trackSoloButton.classList.remove("active");
            channelMuteButton.classList.add("active");
            channelSoloButton.classList.remove("active");
        }

        await updateTrack(track.name, { Mute: !active });
    });

    channelSoloButton.addEventListener("click", async () => {
        var active = trackSoloButton.classList.contains("active");
        if (active) {
            trackSoloButton.classList.remove("active");
            channelSoloButton.classList.remove("active");
        } else {
            trackSoloButton.classList.add("active");
            trackMuteButton.classList.remove("active");
            channelSoloButton.classList.add("active");
            channelMuteButton.classList.remove("active");
        }

        await updateTrack(track.name, { Solo: !active });
    });

    channelRecordButton.addEventListener("click", async () => {
        var active = trackRecordButton.classList.contains("active");
        if (active) {
            trackRecordButton.classList.remove("active");
            channelRecordButton.classList.remove("active");
        } else {
            trackRecordButton.classList.add("active");
            channelRecordButton.classList.add("active");
        }

        await updateTrack(track.name, { Record: !active });
    });

    channelMonitorButton.addEventListener("click", async () => {
        var active = trackMonitorButton.classList.contains("active");
        if (active) {
            trackMonitorButton.classList.remove("active");
            channelMonitorButton.classList.remove("active");
        } else {
            trackMonitorButton.classList.add("active");
            channelMonitorButton.classList.add("active");
        }

        await updateTrack(track.name, { Monitor: !active });
    });

    trackContainer.appendChild(newTrack);
    channelTrackContainer.appendChild(newChannel);
}
import type { TrackInfo, TrackListResponse, TrackUpdate } from "./types.js";
import { invoke } from "@tauri-apps/api/core";
import { percentToDb, percentToGain } from "./utils.js";

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
    const channelFader = newChannel.querySelector(".fader") as HTMLElement;
    const channelFaderThumb = newChannel.querySelector(".fader-thumb") as HTMLElement;

    trackName.textContent = track.name;
    channelName.textContent = track.name;

    // Initialize button active states from server-side track state
    if (track.mute) {
        trackMuteButton.classList.add("active");
        channelMuteButton.classList.add("active");
    }

    if (track.solo) {
        trackSoloButton.classList.add("active");
        channelSoloButton.classList.add("active");
    }

    if (track.record) {
        trackRecordButton.classList.add("active");
        channelRecordButton.classList.add("active");
    }

    if (track.monitor) {
        trackMonitorButton.classList.add("active");
        channelMonitorButton.classList.add("active");
    }

    channelFaderThumb.dataset.dragging = "false";
    channelFaderThumb.dataset.offSetY = "0";

    trackMuteButton.addEventListener("click", async () => {
        const active = trackMuteButton.classList.contains("active");
        const newState = !active;
        try {
            await updateTrack(track.name, { Mute: newState });
        } catch (err) {
            console.error("Failed to update track:", err);
            return;
        }

        if (!newState) {
            trackMuteButton.classList.remove("active");
            channelMuteButton.classList.remove("active");
        } else {
            trackMuteButton.classList.add("active");
            trackSoloButton.classList.remove("active");
            channelMuteButton.classList.add("active");
            channelSoloButton.classList.remove("active");
        }
    });

    trackSoloButton.addEventListener("click", async () => {
        const active = trackSoloButton.classList.contains("active");
        const newState = !active;
        try {
            await updateTrack(track.name, { Solo: newState });
        } catch (err) {
            console.error("Failed to update track:", err);
            return;
        }

        if (!newState) {
            trackSoloButton.classList.remove("active");
            channelSoloButton.classList.remove("active");
        } else {
            trackSoloButton.classList.add("active");
            trackMuteButton.classList.remove("active");
            channelSoloButton.classList.add("active");
            channelMuteButton.classList.remove("active");
        }
    });

    trackRecordButton.addEventListener("click", async () => {
        const active = trackRecordButton.classList.contains("active");
        const newState = !active;
        try {
            await updateTrack(track.name, { Record: newState });
        } catch (err) {
            console.error("Failed to update track:", err);
            return;
        }

        if (!newState) {
            trackRecordButton.classList.remove("active");
            channelRecordButton.classList.remove("active");
        } else {
            trackRecordButton.classList.add("active");
            channelRecordButton.classList.add("active");
        }
    });

    trackMonitorButton.addEventListener("click", async () => {
        const active = trackMonitorButton.classList.contains("active");
        const newState = !active;
        try {
            await updateTrack(track.name, { Monitor: newState });
        } catch (err) {
            console.error("Failed to update track:", err);
            return;
        }

        if (!newState) {
            trackMonitorButton.classList.remove("active");
            channelMonitorButton.classList.remove("active");
        } else {
            trackMonitorButton.classList.add("active");
            channelMonitorButton.classList.add("active");
        }
    });

    channelMuteButton.addEventListener("click", async () => {
        const active = trackMuteButton.classList.contains("active");
        const newState = !active;
        try {
            await updateTrack(track.name, { Mute: newState });
        } catch (err) {
            console.error("Failed to update track:", err);
            return;
        }

        if (!newState) {
            trackMuteButton.classList.remove("active");
            channelMuteButton.classList.remove("active");
        } else {
            trackMuteButton.classList.add("active");
            trackSoloButton.classList.remove("active");
            channelMuteButton.classList.add("active");
            channelSoloButton.classList.remove("active");
        }
    });

    channelSoloButton.addEventListener("click", async () => {
        const active = trackSoloButton.classList.contains("active");
        const newState = !active;
        try {
            await updateTrack(track.name, { Solo: newState });
        } catch (err) {
            console.error("Failed to update track:", err);
            return;
        }

        if (!newState) {
            trackSoloButton.classList.remove("active");
            channelSoloButton.classList.remove("active");
        } else {
            trackSoloButton.classList.add("active");
            trackMuteButton.classList.remove("active");
            channelSoloButton.classList.add("active");
            channelMuteButton.classList.remove("active");
        }
    });

    channelRecordButton.addEventListener("click", async () => {
        const active = trackRecordButton.classList.contains("active");
        const newState = !active;
        try {
            await updateTrack(track.name, { Record: newState });
        } catch (err) {
            console.error("Failed to update track:", err);
            return;
        }

        if (!newState) {
            trackRecordButton.classList.remove("active");
            channelRecordButton.classList.remove("active");
        } else {
            trackRecordButton.classList.add("active");
            channelRecordButton.classList.add("active");
        }
    });

    channelMonitorButton.addEventListener("click", async () => {
        const active = trackMonitorButton.classList.contains("active");
        const newState = !active;
        try {
            await updateTrack(track.name, { Monitor: newState });
        } catch (err) {
            console.error("Failed to update track:", err);
            return;
        }

        if (!newState) {
            trackMonitorButton.classList.remove("active");
            channelMonitorButton.classList.remove("active");
        } else {
            trackMonitorButton.classList.add("active");
            channelMonitorButton.classList.add("active");
        }
    });

    channelFaderThumb.addEventListener("mousedown", (e) => {
        channelFaderThumb.dataset.dragging = "true";
        channelFaderThumb.dataset.offsetY = (e.clientY - channelFaderThumb.getBoundingClientRect().top).toString();
        channelFaderThumb.style.cursor = "ns-resize";
    });

    window.addEventListener("mouseup", () => {
        channelFaderThumb.dataset.dragging = "false";
        channelFaderThumb.style.cursor = "ns-resize";
    });

    window.addEventListener("mousemove", (e) => {
        if (channelFaderThumb.dataset.dragging !== "true") return;
        const faderRect = channelFader.getBoundingClientRect();
        const offsetY = parseFloat(channelFaderThumb.dataset.offsetY ?? "0");
        let newY = e.clientY - faderRect.top - offsetY;
        newY = Math.max(
            channelFaderThumb.offsetHeight / 2,
            Math.min(newY, faderRect.height - channelFaderThumb.offsetHeight / 2)
        );
        const faderRange = faderRect.height - channelFaderThumb.offsetHeight;
        let percent = 100 - ((newY - channelFaderThumb.offsetHeight / 2) / faderRange) * 100;
        percent = Math.round(percent);
        newY = ((100 - percent) / 100) * faderRange + (channelFaderThumb.offsetHeight / 2);
        channelFaderThumb.style.top = `${newY}px`;
        // let gain = percentToGain(percent);
        const gain = Math.pow(10, percentToDb(percent) / 20);
        updateTrack(track.name, { Gain: gain });
    });

    channelFader.addEventListener("wheel", (e) => {
        e.preventDefault();
        const faderRect = channelFader.getBoundingClientRect();
        let currentTop = parseFloat(channelFaderThumb.style.top || "0");
        const faderRange = faderRect.height - channelFaderThumb.offsetHeight;
        let currentPercent = 100 - ((currentTop - channelFaderThumb.offsetHeight / 2) / faderRange) * 100;
        const step = e.shiftKey ? 0.1 : 1;
        const delta = e.deltaY > 0 ? -step : step;
        let percent = Math.max(0, Math.min(currentPercent + delta, 100));
        const newY = ((100 - percent) / 100) * faderRange + (channelFaderThumb.offsetHeight / 2);
        channelFaderThumb.style.top = `${newY}px`;
        // let gain = percentToGain(percent);
        const gain = Math.pow(10, percentToDb(percent) / 20);
        updateTrack(track.name, { Gain: gain });
    });

    trackContainer.appendChild(newTrack);
    channelTrackContainer.appendChild(newChannel);
}
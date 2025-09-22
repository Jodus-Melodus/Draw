const { WebviewWindow, event } = (window as any).__TAURI__;
const { listen, emit } = event;

/**
 * Opens settings page
 */
function openSettings() {
    new WebviewWindow("settings", { url: "settings.html" });
}

// Listen for events
listen("open-settings", () => openSettings());

export type TrackInfo = {
    name: string;
    track_type: string;
    volume: number;
    pan: number;
};

export type TrackListResponse = {
    tracks: TrackInfo[];
};

/**
 * Request track list from Rust
 */
export function getTrackList(): Promise<TrackListResponse> {
    return new Promise((resolve) => {
        // Listen for the response
        listen("track-list-response", (event: any) => {
            resolve(event.payload as TrackListResponse);
        });

        // Emit event to Rust
        emit("get-track-list");
    });
}

export type TrackInfo = {
    name: string;
    trackType: string;
    gain: number;
    pan: number;
    monitor: boolean;
    solo: boolean;
    mute: boolean;
    record: boolean;
};

export type TrackListResponse = {
    tracks: TrackInfo[];
};

export type TrackUpdate = { Pan: number }
    | { Name: string }
    | { Gain: number }
    | { Monitor: boolean }
    | { Solo: boolean }
    | { Mute: boolean }
    | { Record: boolean };

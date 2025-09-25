export type TrackInfo = {
    name: string;
    track_type: string;
    gain: number;
    pan: number;
    monitor: boolean,
    solo: boolean
};

export type TrackListResponse = {
    tracks: TrackInfo[];
};

export type TrackUpdate = | { Pan: number } | { Gain: number } | { Monitor: boolean } | { Solo: boolean };
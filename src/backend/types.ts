export type TrackInfo = {
    name: string;
    track_type: string;
    volume: number;
    pan: number;
    monitor: boolean,
    solo: boolean
};

export type TrackListResponse = {
    tracks: TrackInfo[];
};

export type TrackUpdate = | { Pan: number } | { Volume: number } | { Monitor: boolean } | { Solo: boolean };
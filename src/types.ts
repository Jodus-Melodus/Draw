export type TrackInfo = {
    name: string;
    track_type: string;
    volume: number;
    pan: number;
};

export type TrackListResponse = {
    tracks: TrackInfo[];
};

export type TrackUpdate = | { Pan: number } | { Volume: number };
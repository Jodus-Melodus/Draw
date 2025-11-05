use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::track;

#[derive(Deserialize)]
pub enum TrackUpdate {
    Name(String),
    Monitor(bool),
    Record(bool),
    Pan(f32),
    Gain(f32),
    Mute(bool),
}

pub struct TrackList {
    tracks: HashMap<String, Arc<Mutex<track::tracks::InputTrack>>>,
}

impl TrackList {
    pub fn new() -> Self {
        TrackList {
            tracks: HashMap::new(),
        }
    }

    pub fn get_tracks(&self) -> Vec<Arc<Mutex<track::tracks::InputTrack>>> {
        self.tracks.values().cloned().collect()
    }

    pub fn add_track(&mut self, name: &str, track: track::tracks::InputTrack) {
        self.tracks.insert(name.into(), Arc::new(Mutex::new(track)));
    }

    pub fn add_arc_mut_track(&mut self, name: &str, track: Arc<Mutex<track::tracks::InputTrack>>) {
        self.tracks.insert(name.into(), track);
    }

    pub fn remove_track(&mut self, name: &str) -> Option<Arc<Mutex<track::tracks::InputTrack>>> {
        self.tracks.remove(name)
    }

    pub fn get_track(&self, name: &str) -> Option<Arc<Mutex<track::tracks::InputTrack>>> {
        self.tracks.get(name).cloned()
    }

    pub fn track_list(&self) -> Vec<&String> {
        let mut track_names = self.tracks.keys().collect::<Vec<_>>();
        track_names.sort();
        track_names
    }

    pub fn update_track(&mut self, track_name: &str, update: TrackUpdate) {
        if let Some(track_arc) = self.get_track(track_name) {
            let mut track = track_arc.lock().expect("Failed to lock track");

            match update {
                TrackUpdate::Name(name) => {
                    if let Some(track) = self.remove_track(track_name) {
                        self.add_arc_mut_track(&name, track);
                    } else {
                        eprintln!("No existing track with that name");
                    }
                }
                TrackUpdate::Record(record) => track.record = record,
                TrackUpdate::Monitor(monitor) => track.monitor = monitor,
                TrackUpdate::Pan(pan) => track.pan = pan,
                TrackUpdate::Gain(gain) => track.gain = gain,
                TrackUpdate::Mute(mute) => track.mute = mute,
            }

            if track.monitor || track.record {
                track.source.start_stream();
            } else {
                track.source.stop_stream();
            }
        } else {
            eprintln!("Track {} not found", track_name);
        }
    }

    pub fn from_raw(raw_track_list: HashMap<String, track::raw::InputTrackRaw>) -> Self {
        let mut tracks = HashMap::new();

        for (track_name, raw_track) in raw_track_list {
            tracks.insert(
                track_name,
                Arc::new(Mutex::new(track::tracks::InputTrack::from(raw_track))),
            );
        }

        TrackList { tracks }
    }

    pub fn to_raw(&self) -> HashMap<String, track::raw::InputTrackRaw> {
        self.tracks
            .iter()
            .map(|(key, value)| {
                let audio_track = value.lock().expect("Failed to lock track");
                (key.clone(), track::raw::InputTrackRaw::from(&*audio_track))
            })
            .collect()
    }

    pub fn as_response(&self) -> TrackListResponse {
        let mut tracks = Vec::new();

        for (name, track_mutex) in &self.tracks {
            if let Ok(track) = track_mutex.lock() {
                tracks.push(TrackInfo {
                    name: name.clone(),
                    gain: track.gain,
                    pan: track.pan,
                    monitor: track.monitor,
                    mute: track.mute,
                    solo: false,
                    record: track.record,
                });
            }
        }

        tracks.sort_by(|a, b| a.name.cmp(&b.name));
        TrackListResponse { tracks }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
    pub record: bool,
    pub gain: f32,
    pub pan: f32,
    pub monitor: bool,
    pub solo: bool,
    pub mute: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackListResponse {
    pub tracks: Vec<TrackInfo>,
}

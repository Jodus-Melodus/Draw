use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{atomic::Ordering, Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::track;

#[derive(Deserialize)]
pub enum TrackUpdate {
    Record(bool),
    Pan(f32),
    Gain(f32),
    Monitor(bool),
    Solo(bool),
    Mute(bool),
}

pub struct TrackList {
    tracks: HashMap<String, Arc<Mutex<track::track::AudioTrack>>>,
}

impl TrackList {
    pub fn new() -> Self {
        TrackList {
            tracks: HashMap::new(),
        }
    }

    pub fn add_track(&mut self, name: &str, track: track::track::AudioTrack) {
        self.tracks.insert(name.into(), Arc::new(Mutex::new(track)));
    }

    pub fn remove_track(&mut self, name: &str) {
        if self.tracks.contains_key(name) {
            self.tracks.remove(name);
        }
    }

    pub fn get_track(&self, name: &str) -> Option<Arc<Mutex<track::track::AudioTrack>>> {
        self.tracks.get(name).cloned()
    }

    pub fn track_list(&self) -> Vec<&String> {
        let mut track_names = self.tracks.keys().collect::<Vec<_>>();
        track_names.sort();
        track_names
    }

    pub fn update_track(&mut self, track_name: &str, update: TrackUpdate) {
        let track_arc = self.get_track(track_name).expect("Track not found");
        let mut track = track_arc.lock().expect("Failed to lock track");

        match update {
            TrackUpdate::Pan(pan) => track.pan = pan,
            TrackUpdate::Gain(gain) => track.gain = gain,
            TrackUpdate::Monitor(monitor) => track.monitor = monitor,
            TrackUpdate::Solo(solo) => track.solo = solo,
            TrackUpdate::Mute(mute) => track.mute = mute,
            TrackUpdate::Record(record) => track.record = record,
        }
    }

    pub fn from_raw(raw_track_list: HashMap<String, track::raw::AudioTrackRaw>) -> Self {
        let mut tracks = HashMap::new();

        for (track_name, raw_track) in raw_track_list {
            tracks.insert(
                track_name,
                Arc::new(Mutex::new(track::track::AudioTrack::from(raw_track))),
            );
        }

        TrackList { tracks }
    }

    pub fn to_raw(&self) -> HashMap<String, track::raw::AudioTrackRaw> {
        self.tracks
            .iter()
            .map(|(key, value)| {
                let audio_track = value.lock().expect("Failed to lock track");
                (key.clone(), track::raw::AudioTrackRaw::from(&*audio_track))
            })
            .collect()
    }

    pub fn as_response(&self) -> TrackListResponse {
        let mut tracks = Vec::new();

        for (name, track_mutex) in &self.tracks {
            if let Ok(track) = track_mutex.lock() {
                let track_type_str = match track.track_type {
                    track::track::TrackType::MasterOut => "MasterOut",
                    track::track::TrackType::In => "In",
                };

                tracks.push(TrackInfo {
                    name: name.clone(),
                    track_type: track_type_str.to_string(),
                    gain: track.gain,
                    pan: track.pan,
                    monitor: track.monitor,
                    mute: track.mute,
                    solo: track.solo,
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
    pub track_type: String,
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

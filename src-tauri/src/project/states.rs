use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use cpal::Device;

use crate::{track, types};

#[derive(bincode::Encode, bincode::Decode)]
pub struct StateMixerRaw {
    track_list: HashMap<String, track::raw::AudioTrackRaw>,
}

impl From<StateMixer> for StateMixerRaw {
    fn from(value: StateMixer) -> Self {
        let track_list = value.track_list.lock().expect("Failed to lock track list");
        StateMixerRaw {
            track_list: track_list.to_raw(),
        }
    }
}

pub struct StateMixerGuard(pub Arc<Mutex<StateMixer>>);

#[derive(Clone)]
pub struct StateMixer {
    pub track_list: Arc<Mutex<track::track_list::TrackList>>,
}

impl StateMixer {
    pub fn new(app: &tauri::AppHandle, master_output: Arc<Device>) -> Self {
        let mut track_list = track::track_list::TrackList::new();
        let master_out = track::track::AudioTrack::new(
            "Master-Out",
            track::track::TrackType::MasterOut,
            Some(track::source::StreamSource::new(app, master_output)),
            Arc::new(Mutex::new(track::source::FileSource::new(
                PathBuf::from("master-out.wav"),
                1,
                1,
            ))),
        );
        track_list.add_track("master-out", master_out);
        StateMixer {
            track_list: Arc::new(Mutex::new(track_list)),
        }
    }
}

impl From<StateMixerRaw> for StateMixer {
    fn from(value: StateMixerRaw) -> Self {
        let track_list = value.track_list;
        StateMixer {
            track_list: Arc::new(Mutex::new(track::track_list::TrackList::from_raw(
                track_list,
            ))),
        }
    }
}

#[derive(Clone)]
pub struct StateAudioContext {
    pub input_device_registry: Arc<types::InputDeviceRegistry>,
    pub output_device_registry: Arc<types::OutputDeviceRegistry>,
    pub input_device_index: Arc<AtomicUsize>,
    pub output_device_index: Arc<AtomicUsize>,
}

impl StateAudioContext {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let input_device_registry = Arc::new(types::InputDeviceRegistry::new(&host));
        let output_device_registry = Arc::new(types::OutputDeviceRegistry::new(&host));

        StateAudioContext {
            input_device_registry,
            output_device_registry,
            input_device_index: Arc::new(AtomicUsize::new(0)),
            output_device_index: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn input_device(&self) -> Option<Arc<cpal::Device>> {
        self.input_device_registry
            .get(self.input_device_index.load(Ordering::SeqCst))
    }

    pub fn output_device(&self) -> Option<Arc<cpal::Device>> {
        self.output_device_registry
            .get(self.output_device_index.load(Ordering::SeqCst))
    }
}

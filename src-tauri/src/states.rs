use std::sync::{
    atomic::{AtomicU64, AtomicUsize, Ordering},
    Arc, Mutex,
};

use cpal::Device;

use crate::{
    track::{StreamSource, Track, TrackList, TrackType},
    types::{InputDeviceRegistry, OutputDeviceRegistry},
};

#[derive(Clone)]
pub struct StateMixer {
    pub track_list: Arc<Mutex<TrackList>>,
    pub cursor: Arc<AtomicU64>,
}

impl StateMixer {
    pub fn new(master_output: Device) -> Self {
        let mut track_list = TrackList::new();
        let master_out = Track::new(
            TrackType::MasterOut,
            Box::new(StreamSource::new(master_output)),
        );
        track_list.add_track("master-out", master_out);
        StateMixer {
            track_list: Arc::new(Mutex::new(track_list)),
            cursor: Arc::new(AtomicU64::new(0)),
        }
    }
}

#[derive(Clone)]
pub struct StateAudioContext {
    pub input_device_registry: Arc<InputDeviceRegistry>,
    pub output_device_registry: Arc<OutputDeviceRegistry>,
    pub input_device_index: Arc<AtomicUsize>,
    pub output_device_index: Arc<AtomicUsize>,
}

impl StateAudioContext {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let input_device_registry = Arc::new(InputDeviceRegistry::new(&host));
        let output_device_registry = Arc::new(OutputDeviceRegistry::new(&host));

        StateAudioContext {
            input_device_registry,
            output_device_registry,
            input_device_index: Arc::new(AtomicUsize::new(0)),
            output_device_index: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn input_device(&self) -> Option<&cpal::Device> {
        self.input_device_registry
            .get(self.input_device_index.load(Ordering::SeqCst))
    }

    pub fn output_device(&self) -> Option<&cpal::Device> {
        self.output_device_registry
            .get(self.output_device_index.load(Ordering::SeqCst))
    }
}

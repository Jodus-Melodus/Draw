use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use cpal::Device;

use crate::{
    track,
    types::{InputDeviceRegistry, OutputDeviceRegistry},
};

#[tauri::command]
pub fn get_input_stream_device_list(audio_context: tauri::State<StateAudioContext>) -> Vec<String> {
    let input_device_registry = audio_context.input_device_registry.clone();
    input_device_registry.list()
}

#[derive(bincode::Encode, bincode::Decode)]
pub struct StateMixerRaw {
    track_list: HashMap<String, track::AudioTrackRaw>,
}

#[derive(Clone)]
pub struct StateMixer {
    pub track_list: Arc<Mutex<track::TrackList>>,
    pub playhead: Arc<AtomicU64>,
}

impl StateMixer {
    pub fn new(master_output: Arc<Device>) -> Self {
        let mut track_list = track::TrackList::new();
        let master_out = track::AudioTrack::new(
            track::TrackType::MasterOut,
            Some(track::StreamSource::new(master_output)),
            None,
        );
        track_list.add_track("master-out", master_out);
        StateMixer {
            track_list: Arc::new(Mutex::new(track_list)),
            playhead: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn from_raw(raw_state_mixer: StateMixerRaw) -> Self {
        let track_list = raw_state_mixer.track_list;
        StateMixer {
            track_list: Arc::new(Mutex::new(track::TrackList::from_raw(track_list))),
            playhead: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn to_raw(&self) -> StateMixerRaw {
        let list = self.track_list.lock().expect("Failed to lock track list");

        StateMixerRaw {
            track_list: list.to_raw(),
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

    pub fn input_device(&self) -> Option<Arc<cpal::Device>> {
        self.input_device_registry
            .get(self.input_device_index.load(Ordering::SeqCst))
    }

    pub fn output_device(&self) -> Option<Arc<cpal::Device>> {
        self.output_device_registry
            .get(self.output_device_index.load(Ordering::SeqCst))
    }
}

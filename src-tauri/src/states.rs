use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use cpal::Device;

use crate::types::{
    InputDeviceRegistry, OutputDeviceRegistry, RingBuffer, StreamSource, Track, 
    TrackList, TrackType,
};

pub struct StateMixer {
    track_list: Arc<Mutex<TrackList>>,
}

impl StateMixer {
    pub fn new(master_output: Device) -> Self { // TODO get device stream
        let mut track_list = TrackList::new();
        let output_track_source = StreamSource::new();
        let master_out = Track::new(TrackType::MasterOut, Box::new(output_track_source));
        track_list.add_track("master-out", master_out);
        StateMixer {
            track_list: Arc::new(Mutex::new(track_list)),
        }
    }
}

#[derive(Clone)]
pub struct StateAudioContext {
    pub input_device_registry: Arc<InputDeviceRegistry>,
    pub output_device_registry: Arc<OutputDeviceRegistry>,
    pub input_device_index: Arc<AtomicUsize>,
    pub output_device_index: Arc<AtomicUsize>,
    pub host_id: cpal::HostId,
    pub audio_state: StateAudioRecording,
}

impl StateAudioContext {
    pub fn new(host_id: cpal::HostId) -> Self {
        let host = cpal::host_from_id(host_id).expect("Failed to create host");
        let audio_state = StateAudioRecording {
            recording: Arc::new(AtomicBool::new(false)),
            audio_buffer: Arc::new(Mutex::new(RingBuffer::new())),
        };
        let input_device_registry = Arc::new(InputDeviceRegistry::new(&host));
        let output_device_registry = Arc::new(OutputDeviceRegistry::new(&host));

        StateAudioContext {
            input_device_registry,
            output_device_registry,
            input_device_index: Arc::new(AtomicUsize::new(0)),
            output_device_index: Arc::new(AtomicUsize::new(0)),
            host_id,
            audio_state,
        }
    }
    pub fn host(&self) -> cpal::Host {
        cpal::host_from_id(self.host_id).expect("Failed to get host")
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

#[derive(Clone)]
pub struct StateAudioRecording {
    pub recording: Arc<AtomicBool>,
    pub audio_buffer: Arc<Mutex<RingBuffer>>,
}

impl StateAudioRecording {
    pub fn new() -> Self {
        Self {
            recording: Arc::new(AtomicBool::new(false)),
            audio_buffer: Arc::new(Mutex::new(RingBuffer::new())),
        }
    }
}

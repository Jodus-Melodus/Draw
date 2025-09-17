use std::sync::{
    atomic::{AtomicBool, AtomicUsize},
    Arc, Mutex,
};

use crate::types::{AudioContext, AudioRecordingState, InputDeviceRegistry, OutputDeviceRegistry};

pub fn build_audio_context(host_id: cpal::HostId) -> AudioContext {
    let host = cpal::host_from_id(host_id).expect("Failed to create host");
    let audio_state = AudioRecordingState {
        recording: Arc::new(AtomicBool::new(false)),
        audio_buffer: Arc::new(Mutex::new(Vec::new())),
    };
    let input_device_registry = Arc::new(InputDeviceRegistry::new(&host));
    let output_device_registry = Arc::new(OutputDeviceRegistry::new(&host));

    AudioContext {
        input_device_registry,
        output_device_registry,
        input_device_index: Arc::new(AtomicUsize::new(0)),
        output_device_index: Arc::new(AtomicUsize::new(0)),
        host_id,
        audio_state,
    }
}

use std::sync::{atomic::AtomicBool, Arc};

use crate::types::{AudioContext, AudioState, InputDeviceRegistry};

pub fn build_audio_context(host_id: cpal::HostId) -> AudioContext {
    let host = cpal::host_from_id(host_id).expect("Failed to create host");
    let audio_state = AudioState {
        recording: Arc::new(AtomicBool::new(false)),
    };
    let input_device_registry = Arc::new(InputDeviceRegistry::new(&host));
    AudioContext {
        input_device_registry,
        host_id,
        audio_state,
    }
}

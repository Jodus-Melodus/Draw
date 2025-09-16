use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

use cpal::traits::{DeviceTrait, HostTrait};

#[derive(Clone)]
pub struct AudioContext {
    pub input_device_registry: Arc<InputDeviceRegistry>,
    pub host_id: cpal::HostId,
    pub audio_state: AudioState,
}

impl AudioContext {
    pub fn host(&self) -> cpal::Host {
        cpal::host_from_id(self.host_id).expect("Failed to get host")
    }
}

#[derive(Clone)]
pub struct AudioState {
    pub recording: Arc<AtomicBool>,
}

#[derive(Clone)]
pub struct InputDeviceRegistry {
    devices: HashMap<String, cpal::Device>,
}

impl InputDeviceRegistry {
    pub fn new(host: &cpal::Host) -> Self {
        let mut map = HashMap::new();
        for device in host.input_devices().expect("No input devices available") {
            let name = device.name().unwrap_or_else(|_| "Unknown".into());
            map.insert(name.clone(), device);
        }

        Self { devices: map }
    }

    pub fn get(&self, name: &str) -> Option<&cpal::Device> {
        self.devices.get(name)
    }

    pub fn list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }
}

// TODO add host manager
// TODO add output device registry

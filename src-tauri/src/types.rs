use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

use cpal::traits::{DeviceTrait, HostTrait};

#[derive(Clone)]
pub struct AudioContext {
    pub input_device_registry: Arc<InputDeviceRegistry>,
    pub output_device_registry: Arc<OutputDeviceRegistry>,
    pub input_device_index: Arc<AtomicUsize>,
    pub output_device_index: Arc<AtomicUsize>,
    pub host_id: cpal::HostId,
    pub audio_state: AudioRecordingState,
}

impl AudioContext {
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
pub struct AudioRecordingState {
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

    pub fn get_from_name(&self, name: &str) -> Option<&cpal::Device> {
        self.devices.get(name)
    }

    pub fn get(&self, index: usize) -> Option<&cpal::Device> {
        self.devices.values().nth(index)
    }

    pub fn list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }
}

#[derive(Clone)]
pub struct OutputDeviceRegistry {
    devices: HashMap<String, cpal::Device>,
}

impl OutputDeviceRegistry {
    pub fn new(host: &cpal::Host) -> Self {
        let mut map = HashMap::new();
        for device in host.output_devices().expect("No output devices available") {
            let name = device.name().unwrap_or_else(|_| "Unknown".into());
            map.insert(name.clone(), device);
        }

        Self { devices: map }
    }

    pub fn get_from_name(&self, name: &str) -> Option<&cpal::Device> {
        self.devices.get(name)
    }

    pub fn get(&self, index: usize) -> Option<&cpal::Device> {
        self.devices.values().nth(index)
    }

    pub fn list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }
}

// TODO add host manager

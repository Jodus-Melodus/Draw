use std::{collections::HashMap, sync::Arc};

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host,
};

const RINGBUFFER_SIZE: usize = 48000;

pub struct RingBuffer {
    buffer: Vec<f32>,
    read_idx: usize,
    write_idx: usize,
    full: bool,
}

impl RingBuffer {
    pub fn new() -> Self {
        Self {
            buffer: vec![0.0; RINGBUFFER_SIZE],
            read_idx: 0,
            write_idx: 0,
            full: false,
        }
    }

    pub fn push(&mut self, sample: f32) {
        self.buffer[self.write_idx] = sample;
        self.write_idx = (self.write_idx + 1) % self.buffer.len();
        if self.full {
            self.read_idx = (self.read_idx + 1) % self.buffer.len();
        }
        self.full = self.write_idx == self.read_idx;
    }
    pub fn pop(&mut self) -> Option<f32> {
        if !self.full && self.read_idx == self.write_idx {
            return None;
        }

        let sample = self.buffer[self.read_idx];
        self.read_idx = (self.read_idx + 1) % self.buffer.len();
        self.full = false;
        Some(sample)
    }
}

#[derive(Clone)]
pub struct InputDeviceRegistry {
    devices: HashMap<String, Arc<Device>>,
}

impl InputDeviceRegistry {
    pub fn new(host: &Host) -> Self {
        let mut map = HashMap::new();
        for device in host.input_devices().expect("No input devices available") {
            let name = device.name().unwrap_or_else(|_| "Unknown".into());
            map.insert(name.clone(), Arc::new(device));
        }

        Self { devices: map }
    }

    pub fn get_from_name(&self, name: &str) -> Option<Arc<Device>> {
        self.devices.get(name).cloned()
    }

    pub fn get(&self, index: usize) -> Option<Arc<cpal::Device>> {
        self.devices.values().nth(index).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }
}

#[derive(Clone)]
pub struct OutputDeviceRegistry {
    devices: HashMap<String, Arc<Device>>,
}

impl OutputDeviceRegistry {
    pub fn new(host: &Host) -> Self {
        let mut map = HashMap::new();
        for device in host.output_devices().expect("No output devices available") {
            let name = device.name().unwrap_or_else(|_| "Unknown".into());
            map.insert(name.clone(), Arc::new(device));
        }

        Self { devices: map }
    }

    pub fn get_from_name(&self, name: &str) -> Option<Arc<Device>> {
        self.devices.get(name).cloned()
    }

    pub fn get(&self, index: usize) -> Option<Arc<cpal::Device>> {
        self.devices.values().nth(index).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }
}

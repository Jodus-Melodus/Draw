use std::{collections::HashMap, sync::Arc};

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host,
};

pub const RINGBUFFER_SIZE: usize = 960 * 50;

pub struct RingBuffer {
    buffer: [f32; RINGBUFFER_SIZE],
    write_index: usize,
    read_index: usize,
}

impl RingBuffer {
    pub fn new() -> Self {
        RingBuffer {
            buffer: [0.0; RINGBUFFER_SIZE],
            write_index: 0,
            read_index: 0,
        }
    }

    pub fn write(&mut self, data: &[f32]) {
        for &sample in data {
            self.buffer[self.write_index] = sample;
            self.write_index = (self.write_index + 1) % self.buffer.len();

            if self.write_index == self.read_index {
                self.read_index = (self.read_index + 1) % self.buffer.len();
            }
        }
    }

    pub fn read(&mut self, buffer: &mut [f32]) -> usize {
        let unread = (self.write_index + self.buffer.len() - self.read_index) % self.buffer.len();
        let to_read = unread.min(buffer.len());

        for i in 0..to_read {
            buffer[i] = self.buffer[self.read_index];
            self.read_index = (self.read_index + 1) % self.buffer.len();
        }

        to_read
    }

    pub fn peek(&self, buffer: &mut [f32]) -> usize {
        let unread = (self.write_index + self.buffer.len() - self.read_index) % self.buffer.len();
        let to_read = unread.min(buffer.len());
        let mut index = self.read_index;

        for i in 0..to_read {
            buffer[i] = self.buffer[index];
            index = (index + 1) % self.buffer.len();
        }

        to_read
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

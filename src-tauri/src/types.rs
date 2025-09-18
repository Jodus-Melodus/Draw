use std::{
    collections::HashMap,
    error::Error,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    usize,
};

use cpal::traits::{DeviceTrait, HostTrait};

#[derive(Debug, Clone, Copy)]
pub struct RingBuffer {
    pub buffer: [f32; 48000],
    write_index: usize,
    read_index: usize,
    pub size: usize,
}

impl RingBuffer {
    pub fn new() -> Self {
        RingBuffer {
            buffer: [0.0; 48000],
            write_index: 0,
            read_index: 0,
            size: 48000,
        }
    }

    // Write samples to the buffer, avoiding full-buffer ambiguity
    pub fn write(&mut self, samples: &[f32]) -> usize {
        let mut written = 0;
        let available_space = (self.read_index + self.size - 1 - self.write_index) % self.size;

        for i in 0..samples.len().min(available_space) {
            self.buffer[(self.write_index + i) % self.size] = samples[i];
            written += 1;
        }

        self.write_index = (self.write_index + written) % self.size;
        written
    }

    // Read samples and advance read_index
    pub fn read(&mut self, output: &mut [f32]) -> usize {
        let available = (self.write_index + self.size - self.read_index) % self.size;
        let mut read = 0;

        for i in 0..output.len().min(available) {
            output[i] = self.buffer[(self.read_index + i) % self.size];
            read += 1;
        }

        self.read_index = (self.read_index + read) % self.size;
        read
    }

    // Peek samples without advancing read_index
    pub fn peek(&self, output: &mut [f32]) -> usize {
        let available = (self.write_index + self.size - self.read_index) % self.size;
        let mut count = 0;

        for i in 0..output.len().min(available) {
            output[i] = self.buffer[(self.read_index + i) % self.size];
            count += 1;
        }

        count
    }
}

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
    pub audio_buffer: Arc<Mutex<RingBuffer>>,
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

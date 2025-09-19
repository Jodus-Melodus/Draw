use std::{
    collections::HashMap,
    error::Error,
    fs,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    usize,
};

use cpal::traits::{DeviceTrait, HostTrait};

pub struct RingBuffer {
    buffer: [f32; 48000],
    write_index: usize,
    read_index: usize,
}

impl RingBuffer {
    pub fn new() -> Self {
        RingBuffer {
            buffer: [0.0; 48000],
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
            buffer[i] = self.buffer[self.read_index];
            index = (index + 1) % self.buffer.len();
        }

        to_read
    }
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

pub trait TrackAudioSource: Send + Sync {
    fn read(&mut self, buffer: &mut [f32]) -> usize;
    fn write(&mut self, buffer: &[f32]) -> bool;
}

pub struct StreamSource {
    ring_buffer: RingBuffer,
}

pub struct FileSource {
    reader: hound::WavReader<fs::File>,
    writer: hound::WavWriter<fs::File>,
}

impl TrackAudioSource for StreamSource {
    fn read(&mut self, buffer: &mut [f32]) -> usize {
        self.ring_buffer.read(buffer)
    }

    fn write(&mut self, buffer: &[f32]) -> bool {
        false
    }
}

impl TrackAudioSource for FileSource {
    fn read(&mut self, buffer: &mut [f32]) -> usize {
        for (i, sample) in self.reader.samples::<f32>().take(buffer.len()).enumerate() {
            buffer[i] = sample.unwrap_or(0.0);
        }
        buffer.len()
    }

    fn write(&mut self, buffer: &[f32]) -> bool {
        false
    }
}

pub struct Track {
    pub master: bool,
    pub source: Box<dyn TrackAudioSource + Send>,
    pub volume: f32,
    pub pan: f32,
}

impl Track {
    pub fn new(master: bool, source: Box<dyn TrackAudioSource + Send>) -> Self {
        Track {
            master,
            source,
            volume: 100.0,
            pan: 0.0,
        }
    }
}

pub struct TrackList {
    tracks: HashMap<String, Track>,
}

impl TrackList {
    pub fn new() -> Self {
        TrackList {
            tracks: HashMap::new(),
        }
    }

    pub fn add_track(&mut self, name: &str, track: Track) {
        self.tracks.insert(name.into(), track);
    }

    pub fn remove_track(&mut self, name: &str) {
        if self.tracks.contains_key(name) {
            self.tracks.remove(name);
        }
    }

    pub fn get_track(&self, name: &str) -> Option<&Track> {
        self.tracks.get(name)
    }
}

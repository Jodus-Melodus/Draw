use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bincode::{Decode, Encode};

use crate::{track, types};

#[derive(Clone, Copy, Encode, Decode)]
pub enum TrackType {
    In,
    MasterOut,
}

pub struct AudioTrack {
    pub track_type: TrackType,
    pub stream_source: Option<track::source::StreamSource>,
    pub file_source: Option<Arc<Mutex<track::source::FileSource>>>,
    pub sample_rate: u32,
    pub gain: f32,
    pub pan: f32,
    pub solo: bool,
    pub monitor: bool,
    pub mute: bool,
}

impl AudioTrack {
    pub fn new(
        track_type: TrackType,
        stream_source: Option<track::source::StreamSource>,
        file_source: Option<Arc<Mutex<track::source::FileSource>>>,
    ) -> Self {
        let sample_rate = if let Some(ref stream) = stream_source {
            stream.sample_rate
        } else {
            0
        };

        AudioTrack {
            track_type,
            stream_source,
            file_source,
            sample_rate,
            gain: 0.0,
            pan: 0.0,
            monitor: false,
            solo: false,
            mute: false,
        }
    }

    pub fn save_to_wav(&mut self) {
        if let Some(stream) = &self.stream_source {
            if let Some(file_source) = &self.file_source {
                let ring_buffer_clone = stream.ring_buffer.clone();
                let ring_buffer = ring_buffer_clone.lock().expect("Failed to lock buffer");
                let mut data = vec![0.0f32; types::RINGBUFFER_SIZE];
                let count = ring_buffer.peek(&mut data);

                let file_clone = file_source.clone();
                let mut file = file_clone.lock().unwrap();
                file.save_to_wav(data, count);
                file.close_file();
            }
        }
    }

    pub fn start_recording(&mut self, path: Option<PathBuf>) {
        if let Some(stream) = &mut self.stream_source {
            if self.file_source.is_none() {
                if let Some(path) = path {
                    self.file_source = Some(Arc::new(Mutex::new(track::source::FileSource::new(
                        &path,
                        stream.sample_rate,
                    ))))
                } else {
                    panic!("Expected path");
                }
            }

            if let Some(_file_source) = &self.file_source {
                stream.start_thread();
            } else {
                panic!("Track failed to create file source");
            }
        } else {
            panic!("Track has no stream!");
        }
    }

    pub fn stop_recording(&mut self) {
        if let Some(stream) = &mut self.stream_source {
            if let Some(file_source) = &self.file_source {
                stream.stop_thread();
                let mut file = file_source.lock().unwrap();
                file.close_file();
            } else {
                eprintln!("Track has no file source");
            }
        } else {
            eprintln!("Track has no stream");
        }
    }
}

impl From<track::raw::AudioTrackRaw> for AudioTrack {
    fn from(value: track::raw::AudioTrackRaw) -> Self {
        // TODO implement stream and sample rate save
        AudioTrack {
            track_type: value.track_type,
            stream_source: None,
            file_source: if let Some(file_source_path) = value.file_source_path {
                Some(Arc::new(Mutex::new(track::source::FileSource::new(
                    &PathBuf::from(file_source_path),
                    0,
                ))))
            } else {
                None
            },
            sample_rate: 0,
            gain: value.gain,
            pan: value.pan,
            solo: value.solo,
            monitor: value.monitor,
            mute: value.mute,
        }
    }
}

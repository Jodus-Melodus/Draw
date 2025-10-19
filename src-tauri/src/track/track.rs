use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    track,
    types::{self},
};
use bincode::{Decode, Encode};

#[derive(Clone, Copy, Encode, Decode)]
pub enum TrackType {
    In,
    MasterOut,
}

pub struct AudioTrack {
    pub track_type: TrackType,
    pub stream_source: Option<track::source::StreamSource>,
    pub file_source: Arc<Mutex<track::source::FileSource>>,
    pub sample_rate: u32,
    pub record: bool,
    pub gain: f32,
    pub pan: f32,
    pub solo: bool,
    pub monitor: bool,
    pub mute: bool,
}

impl AudioTrack {
    pub fn new(
        _track_name: &str,
        track_type: TrackType,
        stream_source: Option<track::source::StreamSource>,
        file_source: Arc<Mutex<track::source::FileSource>>,
    ) -> Self {
        let sample_rate = if let Some(ref stream) = stream_source {
            stream.sample_rate
        } else {
            panic!("No audio source");
        };

        if let Some(ref stream) = stream_source {
            let file = file_source.clone();
            let ring_buffer = stream.ring_buffer.clone();

            thread::spawn(move || {
                let mut buffer = [0.0f32; 256];
                loop {
                    let read_count = if let Ok(mut rb) = ring_buffer.lock() {
                        rb.read(&mut buffer)
                    } else {
                        0
                    };

                    if read_count > 0 {
                        let samples = buffer[..read_count].to_vec();
                        if let Ok(mut f) = file.lock() {
                            f.save_to_wav(samples, read_count);
                        } else {
                            eprintln!("Failed to lock file for writing");
                        }
                    } else {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            });
        }

        AudioTrack {
            track_type,
            stream_source,
            file_source,
            sample_rate,
            record: false,
            gain: 0.0,
            pan: 0.0,
            monitor: false,
            solo: false,
            mute: false,
        }
    }

    pub fn save_to_wav(&mut self) {
        if let Some(stream) = &self.stream_source {
            let ring_buffer_clone = stream.ring_buffer.clone();
            let ring_buffer = ring_buffer_clone.lock().expect("Failed to lock buffer");
            let mut data = vec![0.0f32; types::RINGBUFFER_SIZE];
            let count = ring_buffer.peek(&mut data);

            let file_clone = self.file_source.clone();
            let mut file = file_clone.lock().unwrap();
            file.save_to_wav(data, count);
            file.close_file();
        }
    }

    pub fn start_recording(&mut self) {
        if self.record {
            if let Some(stream) = &mut self.stream_source {
                stream.start_thread();
            } else {
                eprintln!("Track has no stream!");
            }
        }
    }

    pub fn stop_recording(&mut self) {
        if self.record {
            if let Some(stream) = &mut self.stream_source {
                stream.stop_thread();
            } else {
                eprintln!("Track has no stream");
            }
            if let Ok(mut file) = self.file_source.lock() {
                file.close_file();
            }
        }
    }
}

impl From<track::raw::AudioTrackRaw> for AudioTrack {
    fn from(value: track::raw::AudioTrackRaw) -> Self {
        // TODO implement stream and sample rate save
        AudioTrack {
            track_type: value.track_type,
            stream_source: None,
            file_source: Arc::new(Mutex::new(track::source::FileSource::new(
                PathBuf::from(value.file_source_path),
                1,
            ))),
            sample_rate: 0,
            record: value.record,
            gain: value.gain,
            pan: value.pan,
            solo: value.solo,
            monitor: value.monitor,
            mute: value.mute,
        }
    }
}

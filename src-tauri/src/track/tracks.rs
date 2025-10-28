use std::path::PathBuf;

use crate::track;

pub struct InputTrack {
    pub source: Box<dyn track::io::source::AudioSource>,
    pub name: String,
    pub pan: f32,
    pub mute: bool,
    pub gain: f32,
    pub record: bool,
    pub monitor: bool,
    // TODO implement solo as feature
}

impl InputTrack {
    pub fn new(name: &str, source: Box<dyn track::io::source::AudioSource>) -> Self {
        InputTrack {
            source,
            name: name.to_string(),
            pan: 0.0,
            mute: false,
            gain: 0.5, 
            record: false,
            monitor: false,
        }
    }
}

impl From<track::raw::InputTrackRaw> for InputTrack {
    fn from(value: track::raw::InputTrackRaw) -> Self {
        InputTrack {
            source: {
                match value.source_type {
                    track::io::source::AudioSourceRaw::File(path) => {
                        Box::new(track::io::source::FileSource::new(PathBuf::from(path)))
                    }
                    track::io::source::AudioSourceRaw::Stream(_) => todo!(),
                }
            },
            name: value.name,
            pan: value.pan,
            mute: false,
            gain: value.gain,
            record: false,
            monitor: false,
        }
    }
}

struct DummySink;
impl track::io::sink::AudioSink for DummySink {
    fn start_stream(&self) {
        panic!("Dummy should never be used");
    }

    fn stop_stream(&self) {
        panic!("Dummy should never be used");
    }
}

pub struct OutputTrack {
    pub gain: f32,
    pub pan: f32,
    pub sink: Box<dyn track::io::sink::AudioSink>,
}

impl OutputTrack {
    pub fn new() -> Self {
        OutputTrack {
            sink: Box::new(DummySink),
            gain: 1.0,
            pan: 0.0,
        }
    }

    pub fn initialize(&mut self, sink: Box<dyn track::io::sink::AudioSink>) {
        self.sink = sink;
    }

    pub fn as_response(&self) -> track::track_list::TrackInfo {
        track::track_list::TrackInfo {
            name: "master-out".to_string(),
            record: false,
            gain: self.gain,
            pan: 0.0,
            monitor: false,
            solo: false,
            mute: false,
        }
    }
}

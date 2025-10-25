use std::path::PathBuf;

use crate::track;


pub struct InputTrack {
    pub source: Box<dyn track::sources::source::AudioSource>,
    pub name: String,
    pub pan: f32,
    pub mute: bool,
    pub gain: f32,
    pub record: bool,
    pub monitor: bool,
    // TODO implement solo as feature
}

impl InputTrack {
    pub fn new(name: &str, source: Box<dyn track::sources::source::AudioSource>) -> Self {
        InputTrack {
            source,
            name: name.to_string(),
            pan: 0.0,
            mute: false,
            gain: 100.0,
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
                    track::sources::source::AudioSourceRaw::File(path) => {
                        Box::new(track::sources::source::FileSource::new(PathBuf::from(path)))
                    }
                    track::sources::source::AudioSourceRaw::Stream(_) => todo!(),
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

pub struct OutputTrack {
    pub sink: Box<dyn track::sources::sink::AudioSink>,
    pub gain: f32,
}

impl OutputTrack {
    pub fn new(sink: Box<dyn track::sources::sink::AudioSink>) -> Self {
        OutputTrack { sink, gain: 100.0 }
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

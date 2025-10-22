use crate::track;

pub struct InputTrack {
    source: Box<dyn track::source::AudioSource>,
    pub name: String,
    pub pan: f32,
    pub mute: bool,
    pub gain: f32,
    pub record: bool,
    pub monitor: bool,
    // TODO implement solo as feature
}

impl InputTrack {
    pub fn new(name: &str, source: Box<dyn track::source::AudioSource>) -> Self {
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

pub struct OutputTrack {
    pub sink: Box<dyn track::source::AudioSink>,
    pub gain: f32,
}

impl OutputTrack {
    pub fn new(sink: Box<dyn track::source::AudioSink>) -> Self {
        OutputTrack { sink, gain: 100.0 }
    }
}

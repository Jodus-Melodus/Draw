use crate::track;

// TODO save stream also
#[derive(bincode::Encode, bincode::Decode)]
pub struct AudioTrackRaw {
    pub track_type: track::track::TrackType,
    pub file_source_path: String,
    pub record: bool,
    pub gain: f32,
    pub pan: f32,
    pub solo: bool,
    pub monitor: bool,
    pub mute: bool,
}

impl From<&track::track::AudioTrack> for AudioTrackRaw {
    fn from(value: &track::track::AudioTrack) -> Self {
        AudioTrackRaw {
            track_type: value.track_type,
            file_source_path: {
                let file = value.file_source.lock().unwrap();
                file.get_path()
            },
            record: value.record,
            gain: value.gain,
            pan: value.pan,
            solo: value.solo,
            monitor: value.monitor,
            mute: value.mute,
        }
    }
}

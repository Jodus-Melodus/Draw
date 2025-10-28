use crate::track;

#[derive(bincode::Encode, bincode::Decode)]
pub struct InputTrackRaw {
    pub name: String,
    pub gain: f32,
    pub pan: f32,
    pub source_type: track::sources::source::AudioSourceRaw,
}

impl From<&track::tracks::InputTrack> for InputTrackRaw {
    fn from(value: &track::tracks::InputTrack) -> Self {
        InputTrackRaw {
            name: value.name.clone(),
            gain: value.gain,
            pan: value.pan,
            source_type: value.source.kind(),
        }
    }
}

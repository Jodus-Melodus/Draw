use std::sync::{atomic::AtomicBool, Arc};

#[derive(Clone)]
pub struct RecordingState {
    pub running: Arc<AtomicBool>,
}

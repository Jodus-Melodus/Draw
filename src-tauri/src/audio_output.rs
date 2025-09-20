use crate::states::StateAudioRecording;

#[tauri::command]
pub fn save_file(audio_recording: tauri::State<StateAudioRecording>) {
    let buffer = audio_recording.audio_buffer.clone();
    let ring_buffer = buffer.lock().expect("Failed to lock");
    let mut ring_buffer_data = [0.0; 48000];
    ring_buffer.peek(&mut ring_buffer_data);
    save_audio_to_file("raw.wav", &ring_buffer_data, ring_buffer_data.len() as u32).unwrap();
}
pub fn save_audio_to_file(path: &str, buffer: &[f32], sample_rate: u32) -> hound::Result<()> {
    let spectogram = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spectogram)?;

    for &sample in buffer {
        let s = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        writer.write_sample(s)?;
    }

    writer.finalize()
}

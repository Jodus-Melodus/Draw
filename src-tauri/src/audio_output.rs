
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
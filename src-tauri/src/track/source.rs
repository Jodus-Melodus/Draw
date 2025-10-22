use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, InputCallbackInfo, OutputCallbackInfo, Stream, SupportedStreamConfig,
};
use hound::{WavReader, WavSpec, WavWriter};

use crate::types::RingBuffer;

pub struct StreamSource {
    recording: Arc<AtomicBool>,
    ring_buffer: Arc<Mutex<RingBuffer>>,
    stream: Arc<Stream>,
    config: SupportedStreamConfig,
}

impl StreamSource {
    pub fn new(device: Arc<Device>) -> Self {
        if !device.supports_input() {
            panic!("Device doesn't support input");
        }

        let recording = Arc::new(AtomicBool::new(false));
        let ring_buffer = Arc::new(Mutex::new(RingBuffer::new()));
        let ring_buffer_clone = ring_buffer.clone();
        let config = device.default_input_config().unwrap();
        let stream = Arc::new(
            device
                .build_input_stream(
                    &config.config(),
                    move |data: &[f32], _: &InputCallbackInfo| {
                        if let Ok(mut rb) = ring_buffer_clone.lock() {
                            rb.write(data);
                        }
                    },
                    move |err| eprintln!("Stream error: {}", err),
                    None,
                )
                .expect("Failed to create input stream"),
        );

        StreamSource {
            recording,
            ring_buffer,
            stream,
            config,
        }
    }

    pub fn start_recording(&mut self) {
        let stream = self.stream.clone();
        let recording = self.recording.clone();
        if let Err(e) = stream.play() {
            eprintln!("Failed to play stream: {}", e);
        }
        recording.store(true, Ordering::Relaxed);

        thread::spawn(move || {
            while recording.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
            }

            if let Err(e) = stream.pause() {
                eprintln!("Failed to pause stream: {}", e);
            }
        });
    }

    pub fn stop_recording(&mut self) {
        self.recording.store(false, Ordering::Relaxed);
    }
}

pub struct FileSource {
    reader: WavReader<BufReader<File>>,
    config: WavSpec,
}

impl FileSource {
    pub fn new(path: PathBuf) -> Self {
        let reader = WavReader::open(path).expect("Failed to open file source");
        let config = reader.spec();
        FileSource { reader, config }
    }
}

pub trait AudioSource: Send {}

impl AudioSource for StreamSource {}

impl AudioSource for FileSource {}

pub struct StreamSink {
    stream: Arc<Stream>,
    config: SupportedStreamConfig,
}

impl StreamSink {
    pub fn new(device: Arc<Device>, ring_buffer: Arc<Mutex<RingBuffer>>) -> Self {
        if !device.supports_output() {
            panic!("Device doesn't support output");
        }

        let ring_buffer_clone = ring_buffer.clone();
        let config = device.default_output_config().unwrap();
        let stream = Arc::new(
            device
                .build_output_stream(
                    &config.config(),
                    move |data: &mut [f32], _: &OutputCallbackInfo| {
                        if let Ok(mut rb) = ring_buffer_clone.lock() {
                            rb.read(data);
                        }
                    },
                    move |err| eprintln!("Stream error: {}", err),
                    None,
                )
                .expect("Failed to create output stream"),
        );

        StreamSink { stream, config }
    }
}

pub struct FileSink {
    writer: Option<WavWriter<BufWriter<File>>>,
    config: WavSpec,
}

impl FileSink {
    pub fn new(path: PathBuf, config: WavSpec) -> Self {
        let writer = WavWriter::create(path, config).ok();
        FileSink { writer, config }
    }

    pub fn save_to_wav(&mut self, data: Vec<f32>, count: usize) {
        if let Some(writer) = &mut self.writer {
            // Ensure we write a multiple of channels. If count is not a multiple of channels,
            // pad the remaining samples with zeros so finalize doesn't fail.
            let ch = self.config.channels as usize;
            let mut to_write = data.into_iter().take(count).collect::<Vec<f32>>();
            let remainder = to_write.len() % ch;
            if remainder != 0 {
                let pad = ch - remainder;
                eprintln!(
                    "save_to_wav: padding {} samples to align to {} channels",
                    pad, ch
                );
                for _ in 0..pad {
                    to_write.push(0.0);
                }
            }

            for sample in to_write.into_iter() {
                let s = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                writer.write_sample(s).expect("Failed to write sample");
            }
        } else {
            eprintln!("Track does not have a writer");
        }
    }

    pub fn close_file(&mut self) {
        if let Some(writer) = self.writer.take() {
            if let Err(e) = writer.finalize() {
                eprintln!("Failed to finalize WAV file: {}", e);
            }
        } else {
            eprintln!("No writer to finalize");
        }
    }
}

pub trait AudioSink: Send {}

impl AudioSink for StreamSink {}

impl AudioSink for FileSink {}

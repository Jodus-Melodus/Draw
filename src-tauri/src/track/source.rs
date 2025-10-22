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
    Device,
};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use tauri::{Emitter, Manager};

use crate::types;

pub struct StreamSource {
    pub recording: Arc<AtomicBool>,
    pub ring_buffer: Arc<Mutex<types::RingBuffer>>,
    pub sample_rate: u32,
    pub channels: u16,
    stream: Arc<cpal::Stream>,
}

impl StreamSource {
    pub fn new(app: &tauri::AppHandle, device: Arc<Device>) -> Self {
        let ring_buffer = Arc::new(Mutex::new(types::RingBuffer::new()));
        let ring_buffer_clone = ring_buffer.clone();
        let window = app.get_webview_window("main").unwrap();

        if device.supports_input() {
            let config = device
                .default_input_config()
                .expect("Failed to get input config");
            let sample_rate = config.sample_rate().0;
            let channels = config.channels();

            let stream = device
                .build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        let mut samples_to_emit: Option<Vec<f32>> = None;

                        if let Ok(mut rb) = ring_buffer_clone.lock() {
                            rb.write(data);
                            eprintln!(
                                "input callback: wrote {} samples to ring buffer",
                                data.len()
                            );
                            samples_to_emit = Some(data.to_vec());
                        } else {
                            eprintln!("input callback: failed to lock ring buffer");
                        }

                        if let Some(samples) = samples_to_emit {
                            if let Err(e) = window.emit("audio-samples", samples) {
                                eprintln!("Failed to emit audio sample: {:?}", e);
                            }
                        }
                    },
                    move |err| eprintln!("Stream error: {}", err),
                    None,
                )
                .expect("Failed to create input stream");

            StreamSource {
                ring_buffer,
                stream: Arc::new(stream),
                recording: Arc::new(AtomicBool::new(false)),
                sample_rate,
                channels,
            }
        } else if device.supports_output() {
            let config = device
                .default_output_config()
                .expect("Failed to get output config");
            let sample_rate = config.sample_rate().0;
            let channels = config.channels();

            let stream = device
                .build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        for sample in data.iter_mut() {
                            *sample = 0.0;
                        }
                    },
                    move |err| eprintln!("Stream error: {}", err),
                    None,
                )
                .expect("Failed to create output stream");

            StreamSource {
                ring_buffer,
                stream: Arc::new(stream),
                recording: Arc::new(AtomicBool::new(false)),
                sample_rate,
                channels,
            }
        } else {
            panic!("Device does not support input or output.")
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
    path: PathBuf,
    reader: Option<WavReader<BufReader<File>>>,
    writer: Option<WavWriter<BufWriter<File>>>,
    channels: u16,
}

impl FileSource {
    pub fn new(path: PathBuf, sample_rate: u32, channels: u16) -> Self {
        let spectogram = WavSpec {
            channels: channels as u16,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        eprintln!("Creating WAV writer for path: {}", path.display());
        let writer = match WavWriter::create(&path, spectogram) {
            Ok(w) => {
                eprintln!("Successfully created WAV writer");
                Some(w)
            }
            Err(e) => {
                eprintln!("Failed to create WAV writer for {}: {}", path.display(), e);
                None
            }
        };

        let reader = if let Ok(r) = WavReader::open(&path) {
            Some(r)
        } else {
            None
        };

        Self {
            path,
            reader,
            writer,
            channels,
        }
    }

    pub fn save_to_wav(&mut self, data: Vec<f32>, count: usize) {
        if let Some(writer) = &mut self.writer {
            // Ensure we write a multiple of channels. If count is not a multiple of channels,
            // pad the remaining samples with zeros so finalize doesn't fail.
            let ch = self.channels as usize;
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
            eprintln!("Track {:?} does not have a writer", self.path);
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

    pub fn get_path(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

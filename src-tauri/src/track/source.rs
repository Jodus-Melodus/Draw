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

            let stream = device
                .build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        let mut samples_to_emit: Option<Vec<f32>> = None;

                        if let Ok(mut rb) = ring_buffer_clone.lock() {
                            rb.write(data);
                            println!("Written samples");
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
            }
        } else if device.supports_output() {
            let config = device
                .default_output_config()
                .expect("Failed to get output config");
            let sample_rate = config.sample_rate().0;

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
            }
        } else {
            panic!("Device does not support input or output.")
        }
    }

    pub fn start_thread(&mut self) {
        let stream = self.stream.clone();
        if let Err(e) = stream.play() {
            eprintln!("Failed to play stream: {}", e);
        }
        println!("Started recording");
        let recording = self.recording.clone();
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

    pub fn stop_thread(&mut self) {
        println!("Stopped recording");
        self.recording.store(false, Ordering::Relaxed);
    }
}

pub struct FileSource {
    path: PathBuf,
    reader: Option<WavReader<BufReader<File>>>,
    writer: Option<WavWriter<BufWriter<File>>>,
}

impl FileSource {
    pub fn new(path: PathBuf, sample_rate: u32) -> Self {
        let spectogram = WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        eprintln!("Creating WAV writer for path: {}", path.display());
        let writer = match WavWriter::create(&path, spectogram) {
            Ok(w) => Some(w),
            Err(e) => panic!("Failed to create WAV writer for {}: {}", path.display(), e),
        };

        // let reader = match WavReader::open(&path) {
        //     Ok(r) => Some(r),
        //     Err(e) => panic!("Failed to create WAV reader for {}: {}", path.display(), e),
        // };
        let reader = None;

        Self {
            path,
            reader,
            writer,
        }
    }

    pub fn save_to_wav(&mut self, data: Vec<f32>, count: usize) {
        if let Some(writer) = &mut self.writer {
            for sample in data.into_iter().take(count) {
                let s = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                writer.write_sample(s).expect("Failed to write sample");
            }
        }
    }

    pub fn close_file(&mut self) {
        if let Some(writer) = self.writer.take() {
            match writer.finalize() {
                Ok(()) => println!("Finalized"),
                Err(e) => eprintln!("Failed to finalize: {}", e),
            }
        }
    }

    pub fn get_path(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

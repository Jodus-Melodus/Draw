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
use plotters::{
    backend,
    chart::ChartBuilder,
    prelude::{IntoDrawingArea, PathElement},
    series::LineSeries,
    style,
};
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

                        // Debug: indicate input callback fired and how many samples
                        eprintln!("input callback: got {} samples", data.len());

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

    pub fn start_thread(&mut self) {
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

    pub fn stop_thread(&mut self) {
        self.recording.store(false, Ordering::Relaxed);
    }

    pub fn graph_recording(&self) {
        let buffer = self.ring_buffer.clone();
        let image = backend::BitMapBackend::new("raw.png", (250, 250)).into_drawing_area();
        image.fill(&style::WHITE).unwrap();

        let ring_buffer = buffer.lock().expect("Failed to lock buffer");

        let mut data = vec![0.0f32; types::RINGBUFFER_SIZE];
        let count = ring_buffer.peek(&mut data);

        let samples: Vec<(usize, f32)> = data
            .iter()
            .take(count)
            .enumerate()
            .map(|(i, &y)| (i, y))
            .collect();

        let y_min = samples
            .iter()
            .map(|&(_, y)| y)
            .fold(f32::INFINITY, f32::min);
        let y_max = samples
            .iter()
            .map(|&(_, y)| y)
            .fold(f32::NEG_INFINITY, f32::max);

        let mut chart = ChartBuilder::on(&image)
            .caption("Raw audio data", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..samples.len(), (y_min * 1.5)..(y_max * 1.5))
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(samples.clone(), &style::BLUE))
            .unwrap()
            .label("waveform")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &style::BLUE));

        image.present().unwrap();
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
        // Convert to absolute path if needed
        let abs_path = if path.is_absolute() {
            path
        } else {
            std::env::current_dir().unwrap_or_else(|e| {
                eprintln!("Failed to get current dir: {}, using '.'", e);
                PathBuf::from(".")
            }).join(path)
        };

        // Ensure parent directory exists with all permissions we need
        if let Some(parent) = abs_path.parent() {
            match std::fs::create_dir_all(parent) {
                Ok(_) => eprintln!("Ensured directory exists: {}", parent.display()),
                Err(e) => eprintln!("Failed to create directory {}: {}", parent.display(), e),
            }

            // Try to verify we can write to the directory
            let test_path = parent.join(".test_write");
            match std::fs::File::create(&test_path) {
                Ok(_) => {
                    eprintln!("Successfully verified write access to directory");
                    let _ = std::fs::remove_file(test_path);
                }
                Err(e) => eprintln!("Warning: Cannot write to directory {}: {}", parent.display(), e),
            }
        }

        let spectogram = WavSpec {
            channels: channels as u16,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        eprintln!("Creating WAV writer for path: {}", abs_path.display());
        let writer = match WavWriter::create(&abs_path, spectogram) {
            Ok(w) => {
                eprintln!("Successfully created WAV writer");
                Some(w)
            }
            Err(e) => {
                eprintln!("Failed to create WAV writer for {}: {}", abs_path.display(), e);
                None
            }
        };

        let reader = if let Ok(r) = WavReader::open(&abs_path) {
            Some(r)
        } else {
            None
        };

        Self {
            path: abs_path,
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

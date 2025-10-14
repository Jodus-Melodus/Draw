use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
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
                            samples_to_emit = Some(data.to_vec());
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
        let recording = self.recording.clone();
        if let Err(e) = stream.play() {
            eprintln!("Failed to play stream: {}", e);
        }
        println!("Started recording");
        recording.store(true, Ordering::Relaxed);

        std::thread::spawn(move || {
            while recording.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(100));
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
}

impl FileSource {
    pub fn new(path: &PathBuf, sample_rate: u32) -> Self {
        let spectogram = WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let reader = if let Ok(r) = WavReader::open(path) {
            Some(r)
        } else {
            None
        };

        let writer = if let Ok(w) = WavWriter::create(path, spectogram) {
            Some(w)
        } else {
            None
        };

        Self {
            path: path.to_path_buf(),
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
        } else {
            panic!("No writer");
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

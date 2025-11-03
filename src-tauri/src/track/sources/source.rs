use std::{
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
    Device, InputCallbackInfo, Stream, SupportedStreamConfig,
};
use hound::{WavReader, WavSpec};
use tauri::{AppHandle, Emitter, Manager};

use crate::types;

pub struct StreamSource {
    pub device_name: String,
    streaming: Arc<AtomicBool>,
    ring_buffer: Arc<Mutex<types::RingBuffer>>,
    stream: Arc<Stream>,
    config: SupportedStreamConfig,
}

impl StreamSource {
    pub fn new(device: Arc<Device>, app: AppHandle) -> Self {
        if !device.supports_input() {
            panic!("Device doesn't support input");
        }
        let window = app.get_webview_window("main").unwrap();
        let device_name = device.name().unwrap();
        let streaming = Arc::new(AtomicBool::new(false));
        let ring_buffer = Arc::new(Mutex::new(types::RingBuffer::new()));
        let ring_buffer_clone = ring_buffer.clone();
        let config = device.default_input_config().unwrap();
        let stream = Arc::new(
            device
                .build_input_stream(
                    &config.config(),
                    move |data: &[f32], _: &InputCallbackInfo| {
                        if let Ok(mut rb) = ring_buffer_clone.lock() {
                            let mut sum = 0.0;
                            let count = data.len() as f32;
                            for &sample in data {
                                rb.push(sample);
                                sum += sample;
                            }
                            window.emit("audio-samples", sum / count).unwrap();
                        }
                    },
                    move |err| eprintln!("Source stream error: {}", err),
                    None,
                )
                .expect("Failed to create input stream"),
        );

        StreamSource {
            device_name,
            streaming,
            ring_buffer,
            stream,
            config,
        }
    }

    pub fn start(&self) {
        let stream = self.stream.clone();
        let streaming = self.streaming.clone();
        if let Err(e) = stream.play() {
            eprintln!("Failed to play stream: {}", e);
        }
        streaming.store(true, Ordering::Relaxed);

        thread::spawn(move || {
            while streaming.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(1));
            }

            if let Err(e) = stream.pause() {
                eprintln!("Failed to pause stream: {}", e);
            }
        });
    }

    pub fn stop(&self) {
        self.streaming.store(false, Ordering::Relaxed);
    }
}

pub struct FileSource {
    pub path: PathBuf,
    streaming: Arc<AtomicBool>,
    ring_buffer: Arc<Mutex<types::RingBuffer>>,
    config: WavSpec,
}

impl FileSource {
    pub fn new(path: PathBuf) -> Self {
        let streaming = Arc::new(AtomicBool::new(false));
        let streaming_clone = Arc::clone(&streaming);
        let ring_buffer = Arc::new(Mutex::new(types::RingBuffer::new()));
        let ring_buffer_clone = Arc::clone(&ring_buffer);

        let reader = WavReader::open(&path).expect("Failed to open file source");
        let config = reader.spec();
        println!("{}", reader.duration());
        let p = path.clone();

        thread::spawn(move || loop {
            if !streaming_clone.load(Ordering::Relaxed) {
                thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }

            let mut reader = WavReader::open(&p).expect("Failed to open file source");
            let mut samples = reader.samples::<f32>();

            while streaming_clone.load(Ordering::Relaxed) {
                if let Some(Ok(s)) = samples.next() {
                    if let Ok(mut rb) = ring_buffer_clone.lock() {
                        rb.push(s * 100.0);
                    }
                } else {
                    println!("EOF reached");
                    break;
                }
            }
        });

        Self {
            path,
            streaming,
            ring_buffer,
            config,
        }
    }

    pub fn start(&self) {
        self.streaming.store(true, Ordering::Relaxed);
    }

    pub fn stop(&self) {
        self.streaming.store(false, Ordering::Relaxed);
    }
}

#[derive(bincode::Encode, bincode::Decode)]
pub enum AudioSourceRaw {
    File(String),
    Stream(String),
}

pub trait AudioSource: Send {
    fn get_ring_buffer(&self) -> Arc<Mutex<types::RingBuffer>>;
    fn start_stream(&self);
    fn stop_stream(&self);
    fn kind(&self) -> AudioSourceRaw;
}

impl AudioSource for StreamSource {
    fn get_ring_buffer(&self) -> Arc<Mutex<types::RingBuffer>> {
        self.ring_buffer.clone()
    }

    fn start_stream(&self) {
        self.start();
    }

    fn stop_stream(&self) {
        self.stop();
    }

    fn kind(&self) -> AudioSourceRaw {
        AudioSourceRaw::Stream(self.device_name.clone())
    }
}

impl AudioSource for FileSource {
    fn get_ring_buffer(&self) -> Arc<Mutex<types::RingBuffer>> {
        self.ring_buffer.clone()
    }

    fn start_stream(&self) {
        self.start();
    }

    fn stop_stream(&self) {
        self.stop();
    }

    fn kind(&self) -> AudioSourceRaw {
        AudioSourceRaw::File(self.path.to_string_lossy().to_string())
    }
}

use std::{
    fs::File,
    io::BufReader,
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

use crate::types;

pub struct StreamSource {
    streaming: Arc<AtomicBool>,
    ring_buffer: Arc<Mutex<types::RingBuffer>>,
    stream: Arc<Stream>,
    config: SupportedStreamConfig,
}

impl StreamSource {
    pub fn new(device: Arc<Device>) -> Self {
        if !device.supports_input() {
            panic!("Device doesn't support input");
        }

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
                            for &sample in data {
                                rb.push(sample);
                            }
                        }
                    },
                    move |err| eprintln!("Stream error: {}", err),
                    None,
                )
                .expect("Failed to create input stream"),
        );

        StreamSource {
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

pub trait AudioSource: Send {
    fn get_ring_buffer(&self) -> Arc<Mutex<types::RingBuffer>>;
    fn start_stream(&self);
    fn stop_stream(&self);
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
}

impl AudioSource for FileSource {
    fn get_ring_buffer(&self) -> Arc<Mutex<types::RingBuffer>> {
        todo!()
    }

    fn start_stream(&self) {
        todo!()
    }

    fn stop_stream(&self) {
        todo!()
    }
}

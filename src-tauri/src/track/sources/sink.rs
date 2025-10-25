use std::{
    fs::File,
    io::BufWriter,
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
    Device, OutputCallbackInfo, Stream, SupportedStreamConfig,
};
use hound::{WavSpec, WavWriter};

use crate::track;

pub struct StreamSink {
    stream: Arc<Stream>,
    streaming: Arc<AtomicBool>,
    config: SupportedStreamConfig,
}

impl StreamSink {
    pub fn new(device: Arc<Device>, track_list: Arc<Mutex<track::track_list::TrackList>>) -> Self {
        if !device.supports_output() {
            panic!("Device doesn't support output");
        }
        let streaming = Arc::new(AtomicBool::new(false));
        let config = device.default_output_config().unwrap();
        let stream = Arc::new(
            device
                .build_output_stream(
                    &config.config(),
                    move |data: &mut [f32], _: &OutputCallbackInfo| {
                        for sample in data.iter_mut() {
                            let mut sum = 0.0;
                            if let Ok(tracks) = track_list.lock() {
                                for track in tracks.get_tracks() {
                                    if let Ok(t) = track.lock() {
                                        let ring_buffer = t.source.get_ring_buffer();
                                        if let Ok(mut rb) = ring_buffer.lock() {
                                            sum += rb.pop().unwrap_or(0.0);
                                        };
                                    }
                                }
                                *sample = sum;
                            }
                        }
                    },
                    move |err| eprintln!("Stream error: {}", err),
                    None,
                )
                .expect("Failed to create output stream"),
        );

        StreamSink {
            stream,
            config,
            streaming,
        }
    }

    fn start(&self) {
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

    fn stop(&self) {
        self.streaming.store(false, Ordering::Relaxed);
    }
}

pub struct FileSink {
    writer: Option<WavWriter<BufWriter<File>>>,
    config: WavSpec,
}

impl FileSink {
    pub fn new(path: PathBuf, config: WavSpec) -> Self {
        let writer = WavWriter::create(path, config).ok();
        FileSink {
            writer,
            config,
        }
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

pub trait AudioSink: Send {
    fn start_stream(&self);
    fn stop_stream(&self);
}

impl AudioSink for StreamSink {
    fn start_stream(&self) {
        self.start();
    }

    fn stop_stream(&self) {
        self.stop();
    }
}

impl AudioSink for FileSink {
    fn start_stream(&self) {
        todo!()
    }

    fn stop_stream(&self) {
        todo!()
    }
}

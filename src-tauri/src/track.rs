use std::{
    collections::HashMap,
    fs,
    io::{BufReader, BufWriter},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use cpal::traits::{DeviceTrait, StreamTrait};
use plotters::{
    backend,
    chart::ChartBuilder,
    prelude::{IntoDrawingArea, PathElement},
    series::LineSeries,
    style,
};
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};

use crate::{states, types::{RingBuffer, RINGBUFFER_SIZE}};

#[tauri::command]
pub fn get_track_list(
    state: tauri::State<states::StateMixerGuard>,
) -> Result<TrackListResponse, String> {
    let state_mixer = state.0.lock().map_err(|_| "Failed to lock state mixer")?;
    let list = state_mixer
        .track_list
        .lock()
        .map_err(|_| "Failed to lock track list")?;
    Ok(list.as_response())
}

#[tauri::command]
pub fn update_track(
    state: tauri::State<states::StateMixerGuard>,
    track_name: String,
    update: TrackUpdate,
) -> Result<(), String> {
    let state_mixer = state.0.lock().map_err(|_| "Failed to lock state mixer")?;
    let mut list = state_mixer
        .track_list
        .lock()
        .map_err(|_| "Failed to lock track list")?;
    list.update_track(&track_name, update);
    Ok(())
}

pub struct StreamSource {
    ring_buffer: Arc<Mutex<RingBuffer>>,
    stream: Arc<cpal::Stream>,
    recording: Arc<AtomicBool>,
}

impl StreamSource {
    pub fn new(app: &tauri::AppHandle, device: Arc<cpal::Device>) -> Self {
        let ring_buffer = Arc::new(Mutex::new(RingBuffer::new()));
        let ring_buffer_clone = ring_buffer.clone();
        let window = app.get_webview_window("main").unwrap();

        if device.supports_input() {
            let config = device
                .default_input_config()
                .expect("Failed to get input config");

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
            }
        } else if device.supports_output() {
            let config = device
                .default_output_config()
                .expect("Failed to get output config");

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
        self.save_to_wav("testing.wav");
    }

    pub fn graph_recording(&self) {
        let buffer = self.ring_buffer.clone();
        let image = backend::BitMapBackend::new("raw.png", (250, 250)).into_drawing_area();
        image.fill(&style::WHITE).unwrap();

        let ring_buffer = buffer.lock().expect("Failed to lock buffer");

        let mut data = [0.0; RINGBUFFER_SIZE];
        ring_buffer.peek(&mut data);

        let samples: Vec<(usize, f32)> = data.iter().enumerate().map(|(i, &y)| (i, y)).collect();

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

    pub fn save_to_wav(&mut self, path: &str) {
        let ring_buffer_clone = self.ring_buffer.clone();
        let ring_buffer = ring_buffer_clone.lock().expect("Failed to lock buffer");
        let mut data = vec![0.0; RINGBUFFER_SIZE];
        ring_buffer.peek(&mut data);

        let spectogram = hound::WavSpec {
            channels: 1,
            sample_rate: data.len() as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer =
            hound::WavWriter::create(path, spectogram).expect("Failed to create file writer");
        for sample in data {
            let s = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            writer.write_sample(s).expect("Failed to write sample");
        }

        writer.finalize().expect("Failed to save file");
    }
}

pub struct FileSource {
    path: PathBuf,
    reader: Option<hound::WavReader<BufReader<fs::File>>>,
    writer: Option<hound::WavWriter<BufWriter<fs::File>>>,
}

impl FileSource {
    pub fn new_input(input_path: &PathBuf) -> Self {
        let reader = hound::WavReader::open(input_path).expect("Failed to create reader"); // TODO use absolute path
        Self {
            path: input_path.to_path_buf(),
            reader: Some(reader),
            writer: None,
        }
    }

    pub fn get_path(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

#[derive(Clone, Copy, bincode::Encode, bincode::Decode)]
pub enum TrackType {
    In,
    MasterOut,
}

#[derive(Deserialize)]
pub enum TrackUpdate {
    Pan(f32),
    Gain(f32),
    Monitor(bool),
    Solo(bool),
    Mute(bool),
    Record(bool),
}

pub struct AudioTrack {
    pub track_type: TrackType,
    pub stream_source: Option<StreamSource>,
    pub file_source: Option<FileSource>,
    pub gain: f32,
    pub pan: f32,
    pub solo: bool,
    pub monitor: bool,
    pub mute: bool,
}

impl AudioTrack {
    pub fn new(
        track_type: TrackType,
        stream_source: Option<StreamSource>,
        file_source: Option<FileSource>,
    ) -> Self {
        AudioTrack {
            track_type,
            stream_source,
            file_source,
            gain: 0.0,
            pan: 0.0,
            monitor: false,
            solo: false,
            mute: false,
        }
    }
}

impl From<AudioTrackRaw> for AudioTrack {
    fn from(value: AudioTrackRaw) -> Self {
        AudioTrack {
            track_type: value.track_type,
            stream_source: None,
            file_source: if let Some(file_source_path) = value.file_source_path {
                Some(FileSource::new_input(&PathBuf::from(file_source_path)))
            } else {
                None
            },
            gain: value.gain,
            pan: value.pan,
            solo: value.solo,
            monitor: value.monitor,
            mute: value.mute,
        }
    }
}

// TODO save stream also
#[derive(bincode::Encode, bincode::Decode)]
pub struct AudioTrackRaw {
    track_type: TrackType,
    file_source_path: Option<String>,
    gain: f32,
    pan: f32,
    solo: bool,
    monitor: bool,
    mute: bool,
}

impl From<&AudioTrack> for AudioTrackRaw {
    fn from(value: &AudioTrack) -> Self {
        AudioTrackRaw {
            track_type: value.track_type,
            file_source_path: if let Some(file_source) = &value.file_source {
                Some(file_source.get_path())
            } else {
                None
            },
            gain: value.gain,
            pan: value.pan,
            solo: value.solo,
            monitor: value.monitor,
            mute: value.mute,
        }
    }
}

pub struct TrackList {
    tracks: HashMap<String, Arc<Mutex<AudioTrack>>>,
}

impl TrackList {
    pub fn new() -> Self {
        TrackList {
            tracks: HashMap::new(),
        }
    }

    pub fn add_track(&mut self, name: &str, track: AudioTrack) {
        self.tracks.insert(name.into(), Arc::new(Mutex::new(track)));
    }

    pub fn remove_track(&mut self, name: &str) {
        if self.tracks.contains_key(name) {
            self.tracks.remove(name);
        }
    }

    pub fn get_track(&self, name: &str) -> Option<Arc<Mutex<AudioTrack>>> {
        self.tracks.get(name).cloned()
    }

    pub fn track_list(&self) -> Vec<&String> {
        let mut track_names = self.tracks.keys().collect::<Vec<_>>();
        track_names.sort();
        track_names
    }

    pub fn update_track(&mut self, track_name: &str, update: TrackUpdate) {
        let track_arc = self.get_track(track_name).expect("Track not found");
        let mut track = track_arc.lock().expect("Failed to lock track");

        match update {
            TrackUpdate::Pan(pan) => track.pan = pan,
            TrackUpdate::Gain(gain) => track.gain = gain,
            TrackUpdate::Monitor(monitor) => track.monitor = monitor,
            TrackUpdate::Solo(solo) => track.solo = solo,
            TrackUpdate::Mute(mute) => track.mute = mute,
            TrackUpdate::Record(record) => {
                if let Some(stream) = &mut track.stream_source {
                    match record {
                        true => stream.start_thread(),
                        false => stream.stop_thread(),
                    }
                }
            }
        }
    }

    pub fn from_raw(raw_track_list: HashMap<String, AudioTrackRaw>) -> Self {
        let mut tracks = HashMap::new();

        for (track_name, raw_track) in raw_track_list {
            tracks.insert(
                track_name,
                Arc::new(Mutex::new(AudioTrack::from(raw_track))),
            );
        }

        TrackList { tracks }
    }

    pub fn to_raw(&self) -> HashMap<String, AudioTrackRaw> {
        self.tracks
            .iter()
            .map(|(key, value)| {
                let audio_track = value.lock().expect("Failed to lock track");
                (key.clone(), AudioTrackRaw::from(&*audio_track))
            })
            .collect()
    }

    pub fn as_response(&self) -> TrackListResponse {
        let mut tracks = Vec::new();

        for (name, track_mutex) in &self.tracks {
            if let Ok(track) = track_mutex.lock() {
                let track_type_str = match track.track_type {
                    TrackType::In => "In",
                    TrackType::MasterOut => "MasterOut",
                };

                let record = if let Some(recording) = &track.stream_source {
                    let r = recording.recording.clone();
                    r.load(Ordering::Relaxed)
                } else {
                    false
                };

                tracks.push(TrackInfo {
                    name: name.clone(),
                    track_type: track_type_str.to_string(),
                    gain: track.gain,
                    pan: track.pan,
                    monitor: track.monitor,
                    mute: track.mute,
                    solo: track.solo,
                    record,
                });
            }
        }

        tracks.sort_by(|a, b| a.name.cmp(&b.name));

        TrackListResponse { tracks }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
    pub track_type: String,
    pub gain: f32,
    pub pan: f32,
    pub monitor: bool,
    pub solo: bool,
    pub mute: bool,
    pub record: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackListResponse {
    pub tracks: Vec<TrackInfo>,
}

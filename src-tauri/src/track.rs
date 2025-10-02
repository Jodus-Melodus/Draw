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

use crate::{states, types::RingBuffer};

#[tauri::command]
pub fn get_track_list(state: tauri::State<states::StateMixerGuard>) -> TrackListResponse {
    let state_mixer = state.0.lock().unwrap();
    let track_list = state_mixer.track_list.clone();
    let list = track_list.lock().expect("Failed to lock list");
    list.as_response()
}

#[tauri::command]
pub fn update_track(
    state: tauri::State<states::StateMixerGuard>,
    track_name: String,
    update: TrackUpdate,
) {
    let state_mixer = state.0.lock().unwrap();
    let track_list = state_mixer.track_list.clone();
    let mut list = track_list.lock().expect("Failed to lock list");
    list.update_track(&track_name, update);
}

pub struct StreamSource {
    ring_buffer: Arc<Mutex<RingBuffer>>,
    stream: Arc<cpal::Stream>,
    recording: Arc<AtomicBool>,
}

impl StreamSource {
    pub fn new(device: Arc<cpal::Device>) -> Self {
        let ring_buffer = Arc::new(Mutex::new(RingBuffer::new()));
        let ring_buffer_clone = ring_buffer.clone();

        if device.supports_input() {
            let config = device
                .default_input_config()
                .expect("Failed to get input config");

            let stream = device
                .build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        if let Ok(mut rb) = ring_buffer_clone.lock() {
                            rb.write(data);
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
            panic!("")
        }
    }

    pub fn start_thread(&mut self) {
        let stream = self.stream.clone();
        let recording = self.recording.clone();

        recording.store(true, Ordering::Relaxed);

        std::thread::spawn(move || {
            while recording.load(Ordering::Relaxed) {
                if let Err(e) = stream.play() {
                    eprintln!("Failed to play stream: {}", e);
                }
                std::thread::sleep(Duration::from_millis(100));
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

        let mut data = [0.0; 48000];
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
        println!("Saved waveform to raw.png");
    }

    pub fn save_to_wav(&mut self, path: &str) {
        let ring_buffer = self.ring_buffer.clone();
        let buffer = ring_buffer.lock().expect("Failed to lock buffer");
        let mut data = [0.0; 48000];
        buffer.peek(&mut data);

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
}

pub struct AudioTrack {
    pub track_type: TrackType,
    pub stream_source: Option<StreamSource>,
    pub file_source: Option<FileSource>,
    pub gain: f32,
    pub pan: f32,
    pub solo: bool,
    pub monitor: bool,
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
        self.tracks.keys().collect::<Vec<_>>()
    }

    pub fn update_track(&mut self, track_name: &str, update: TrackUpdate) {
        let track_arc = self.get_track(track_name).expect("Track not found");
        let mut track = track_arc.lock().expect("Failed to lock track");

        match update {
            TrackUpdate::Pan(pan) => track.pan = pan,
            TrackUpdate::Gain(gain) => track.gain = gain,
            TrackUpdate::Monitor(monitor) => track.monitor = monitor,
            TrackUpdate::Solo(solo) => track.solo = solo,
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

                tracks.push(TrackInfo {
                    name: name.clone(),
                    track_type: track_type_str.to_string(),
                    gain: track.gain,
                    pan: track.pan,
                });
            }
        }

        TrackListResponse { tracks }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
    pub track_type: String,
    pub gain: f32,
    pub pan: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackListResponse {
    pub tracks: Vec<TrackInfo>,
}

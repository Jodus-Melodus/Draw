use plotters::{
    backend,
    chart::ChartBuilder,
    prelude::{IntoDrawingArea, PathElement},
    series::LineSeries,
    style,
};
use std::sync::atomic::Ordering;

use crate::{
    states::{StateAudioContext, StateAudioRecording, StateMixer},
    types::{StreamSource, Track, TrackType},
};

#[tauri::command]
pub fn start_audio_input(
    audio_context: tauri::State<StateAudioContext>,
    audio_recording: tauri::State<StateAudioRecording>,
    mixer_state: tauri::State<StateMixer>,
) {
    let audio_recording = audio_recording.clone();
    let mixer = mixer_state.clone();
    let track_list = mixer.track_list.clone();

    if audio_recording.recording.load(Ordering::Relaxed) {
        println!("Audio stream already running");
        return;
    };

    let recording = audio_recording.recording.clone();
    recording.store(true, Ordering::Relaxed);
    let device = audio_context
        .input_device()
        .expect("Failed to get input device")
        .clone();
    let track = Track::new(TrackType::MasterIn, Box::new(StreamSource::new(device)));
    // TODO start track stream
}

#[tauri::command]
pub fn stop_audio_input(audio_recording: tauri::State<StateAudioRecording>) {
    audio_recording.recording.store(false, Ordering::Relaxed);
}

#[tauri::command]
pub fn graph_recording(audio_recording: tauri::State<StateAudioRecording>) {
    let buffer = audio_recording.audio_buffer.clone();
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

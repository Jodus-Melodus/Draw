use cpal::traits::{DeviceTrait, StreamTrait};
use plotters::{
    backend,
    chart::ChartBuilder,
    prelude::{IntoDrawingArea, PathElement},
    series::LineSeries,
    style,
};
use std::{
    sync::{atomic::Ordering, Arc, Mutex},
    thread,
};

use crate::types::{AudioContext, RingBuffer};

#[tauri::command]
pub fn start_audio_input(state: tauri::State<AudioContext>) {
    let audio_state = state.audio_state.clone();

    if audio_state.recording.load(Ordering::Relaxed) {
        println!("Audio stream already running");
        return;
    }

    audio_state.recording.store(true, Ordering::Relaxed);
    let recording = audio_state.recording.clone();
    let device = state
        .input_device()
        .expect("Failed to get input device")
        .clone();
    println!("Recording with: {}", device.name().unwrap());

    thread::spawn(move || {
        let config = device
            .default_input_config()
            .expect("Failed to get input config");
        let buffer = audio_state.audio_buffer.clone();

        let stream = device
            .build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut ring_buffer = buffer.lock().expect("Failed to lock buffer");
                    ring_buffer.write(data);
                },
                move |err| eprintln!("Stream error: {}", err),
                None,
            )
            .expect("Failed to create input stream");

        stream.play().expect("Failed to play stream");

        while recording.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        println!("Audio stream stopped.");
    });
}

#[tauri::command]
pub fn stop_audio_input(state: tauri::State<AudioContext>) {
    state.audio_state.recording.store(false, Ordering::Relaxed);
}

#[tauri::command]
pub fn graph_recording(state: tauri::State<AudioContext>) {
    let buffer = state.audio_state.audio_buffer.clone();
    let image = backend::BitMapBackend::new("raw.png", (640, 480)).into_drawing_area();
    image.fill(&style::WHITE).unwrap();

    let ring_buffer = buffer.lock().expect("Failed to lock buffer");

    // Peek instead of read so buffer is not emptied
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
        .build_cartesian_2d(0..samples.len(), y_min..y_max)
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

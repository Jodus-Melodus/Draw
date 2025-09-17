use cpal::traits::{DeviceTrait, StreamTrait};
use std::{sync::atomic::Ordering, thread};

use crate::types::AudioContext;

#[tauri::command]
pub fn start_audio_input(state: tauri::State<AudioContext>) {
    let audio_state = state.audio_state.clone();
    let input_device_registry = state.input_device_registry.clone();

    if audio_state.recording.load(Ordering::SeqCst) {
        println!("Audio stream already running");
        return;
    }

    audio_state.recording.store(true, Ordering::SeqCst);
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

        let stream = device
            .build_input_stream(
                &config.into(),
                move |_data: &[f32], _: &cpal::InputCallbackInfo| {
                    // println!("Received {} samples", data.len()); // TODO write the data to a global Arc<ArrayQueue<f32>> for processing
                },
                move |err| eprintln!("Stream error: {}", err),
                None,
            )
            .expect("Failed to create input stream");

        stream.play().expect("Failed to play stream");

        while recording.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        println!("Audio stream stopped.");
    });
}

#[tauri::command]
pub fn stop_audio_input(state: tauri::State<AudioContext>) {
    state.audio_state.recording.store(false, Ordering::SeqCst);
}

// TODO save audio to file
// TODO read audio from file

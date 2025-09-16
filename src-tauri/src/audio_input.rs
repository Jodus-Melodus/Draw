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

    thread::spawn(move || {
        let devices = input_device_registry.list();
        if devices.is_empty() {
            eprintln!("No input devices available");
            return;
        }

        let device = input_device_registry
            .get(&devices[0])
            .expect("Failed to get input device");

        let config = device
            .default_input_config()
            .expect("Failed to get input config");

        let stream = device
            .build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    println!("Received {} samples", data.len()); // TODO write the data to a global Arc<ArrayQueue<f32>> for processing
                },
                move |err| eprintln!("Stream error: {}", err),
                None,
            )
            .expect("Failed to create input stream");

        stream.play().expect("Failed to play stream");

        while recording.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
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

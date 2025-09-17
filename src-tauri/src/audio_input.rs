use cpal::traits::{DeviceTrait, StreamTrait};
use std::{
    sync::{atomic::Ordering, Arc, Mutex, MutexGuard},
    thread,
};

use crate::types::AudioContext;

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
                    let mut vec = buffer.lock().expect("Failed to lock buffer");
                    *vec = data.to_vec();
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

// TODO save audio to file
// TODO read audio from file
// TODO save stream handle in audiorecording state

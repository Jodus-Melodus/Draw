use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{sync::atomic::Ordering, thread};

use crate::types::RecordingState;

#[tauri::command]
pub fn start_audio_input(state: tauri::State<RecordingState>) {
    if state.running.load(Ordering::SeqCst) {
        println!("Audio stream already running");
        return;
    }

    state.running.store(true, Ordering::SeqCst);
    let running = state.running.clone();

    thread::spawn(move || {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("No input device available");
        let config = device.default_input_config().unwrap();

        let stream = device
            .build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    println!("Received {} samples", data.len());
                },
                move |err| eprintln!("Stream error: {}", err),
                None,
            )
            .unwrap();

        stream.play().unwrap();

        while running.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        println!("Audio stream stopped.");
    });
}

#[tauri::command]
pub fn stop_audio_input(state: tauri::State<RecordingState>) {
    state.running.store(false, Ordering::SeqCst);
}

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use cpal::Device;
use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};

use crate::{track, types};

#[derive(bincode::Encode, bincode::Decode)]
pub struct StateMixerRaw {
    track_list: HashMap<String, track::raw::InputTrackRaw>,
}

impl From<&StateMixer> for StateMixerRaw {
    fn from(value: &StateMixer) -> Self {
        let track_list = value.track_list.lock().expect("Failed to lock track list");
        StateMixerRaw {
            track_list: track_list.to_raw(),
        }
    }
}

pub struct StateMixerGuard(pub Arc<Mutex<StateMixer>>);

pub struct StateMixer {
    pub track_list: Arc<Mutex<track::track_list::TrackList>>,
    pub master_out: Arc<Mutex<track::tracks::OutputTrack>>,
    pub discord_client: Mutex<DiscordIpcClient>,
}

impl StateMixer {
    pub fn new(device: Arc<Device>) -> Self {
        let track_list = Arc::new(Mutex::new(track::track_list::TrackList::new()));
        let master_out = Arc::new(Mutex::new(track::tracks::OutputTrack::new()));
        let sink =
            track::sources::sink::StreamSink::new(device, track_list.clone(), master_out.clone());
        if let Ok(mut out) = master_out.lock() {
            out.initialize(Box::new(sink));
            out.sink.start_stream();
        }
        let discord_client = Mutex::new(DiscordIpcClient::new("1435880809767637164"));

        let sm = StateMixer {
            track_list,
            master_out,
            discord_client,
        };
        sm.connect_to_discord();
        sm
    }

    pub fn connect_to_discord(&self) {
        if let Ok(mut client) = self.discord_client.lock() {
            if let Err(e) = client.connect() {
                eprintln!("Failed to connect to Discord: {}", e);
            } else {
                self.set_discord_activity("DRAW", "Making some noise with DRAW");
            }
        }
    }

    pub fn set_discord_activity(&self, state: &str, details: &str) {
        if let Ok(mut client) = self.discord_client.lock() {
            if let Err(e) = client.set_activity(Activity::new().state(state).details(details)) {
                eprintln!("Failed to set Discord activity: {}", e);
            }
        }
    }

    pub fn disconnect_from_discord(&self) {
        if let Ok(mut client) = self.discord_client.lock() {
            if let Err(e) = client.close() {
                eprintln!("Failed to close Discord connection : {}", e);
            }
        }
    }
}

impl From<StateMixerRaw> for StateMixer {
    fn from(value: StateMixerRaw) -> Self {
        let track_list = value.track_list;
        let discord_client = Mutex::new(DiscordIpcClient::new("1435880809767637164"));
        StateMixer {
            track_list: Arc::new(Mutex::new(track::track_list::TrackList::from_raw(
                track_list,
            ))),
            discord_client,
            master_out: { todo!() },
        }
    }
}

#[derive(Clone)]
pub struct StateAudioContext {
    pub input_device_registry: Arc<types::InputDeviceRegistry>,
    pub output_device_registry: Arc<types::OutputDeviceRegistry>,
    pub input_device_index: Arc<AtomicUsize>,
    pub output_device_index: Arc<AtomicUsize>,
}

impl StateAudioContext {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let input_device_registry = Arc::new(types::InputDeviceRegistry::new(&host));
        let output_device_registry = Arc::new(types::OutputDeviceRegistry::new(&host));

        StateAudioContext {
            input_device_registry,
            output_device_registry,
            input_device_index: Arc::new(AtomicUsize::new(0)),
            output_device_index: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn input_device(&self) -> Option<Arc<cpal::Device>> {
        self.input_device_registry
            .get(self.input_device_index.load(Ordering::SeqCst))
    }

    pub fn output_device(&self) -> Option<Arc<cpal::Device>> {
        self.output_device_registry
            .get(self.output_device_index.load(Ordering::SeqCst))
    }
}

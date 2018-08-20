extern crate serialport;

use std::sync::{Arc, Mutex};

use super::AudioFilter;
use super::Mixer;
use super::super::serial_api::SerialApi;

pub struct ComMixer {
    serial_api: Arc<Mutex<SerialApi>>
}

impl ComMixer {
    pub fn new(serial_api: Arc<Mutex<SerialApi>>) -> ComMixer {
        ComMixer {
            serial_api: serial_api
        }
    }
}

impl Mixer for ComMixer {
    fn start(&mut self) {
        if let Ok(ref mut mutex) = self.serial_api.try_lock() {
            mutex.write("start");
        }
    }

    fn stop(&mut self) {
        if let Ok(ref mut mutex) = self.serial_api.try_lock() {
            mutex.write("stop");
        }
    }

    fn set_volume(&mut self, volume: u16) {
        print!("setVolume {}", volume);
    }

    fn volume(&mut self) -> u16 {
        return 0
    }

    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
}
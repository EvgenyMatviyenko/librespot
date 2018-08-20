use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use super::AudioFilter;
use super::Mixer;

#[derive(Clone)]
pub struct SoftMixer {
    volume: Arc<AtomicUsize>,
}

impl Mixer for SoftMixer {
    fn start(&mut self) {}
    fn stop(&mut self) {}
    fn volume(&mut self) -> u16 {
        self.volume.load(Ordering::Relaxed) as u16
    }
    fn set_volume(&mut self, volume: u16) {
        self.volume.store(volume as usize, Ordering::Relaxed);
    }
    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        Some(Box::new(SoftVolumeApplier {
            volume: self.volume.clone(),
        }))
    }
}

struct SoftVolumeApplier {
    volume: Arc<AtomicUsize>,
}

impl AudioFilter for SoftVolumeApplier {
    fn modify_stream(&self, data: &mut [i16]) {
        let volume = self.volume.load(Ordering::Relaxed) as u16;
        if volume != 0xFFFF {
            for x in data.iter_mut() {
                *x = (*x as i32 * volume as i32 / 0xFFFF) as i16;
            }
        }
    }
}
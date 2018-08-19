use super::AudioFilter;
use super::Mixer;

#[derive(Clone)]
pub struct ComMixer {

}

impl Mixer for ComMixer {
    fn open() -> ComMixer {
        ComMixer { }
    }

    fn start(&self) {
        print!("start");
    }

    fn stop(&self) {
        print!("stop");
    }

    fn set_volume(&self, volume: u16) {
        print!("setVolume {}", volume);
    }

    fn volume(&self) -> u16 {
        return 0
    }

    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
}
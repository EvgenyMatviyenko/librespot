pub trait Mixer: Send {
    fn start(&mut self);
    fn stop(&mut self);
    fn set_volume(&mut self, volume: u16);
    fn volume(&mut self) -> u16;
    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
}

pub trait AudioFilter {
    fn modify_stream(&self, data: &mut [i16]);
}

pub mod softmixer;
use self::softmixer::SoftMixer;

pub mod commixer;
use self::commixer::ComMixer;
use std::cell::RefCell;
use std::rc::Rc;
use qam;
use audio;

pub struct Physical {
    modulator: qam::Modulator,
    demodulator: qam::Demodulator,
    audio: Rc<RefCell<audio::Audio>>
}

impl Physical {
    pub fn new() -> Physical {
        let mut audio = Rc::new(RefCell::new(audio::Audio::new()));

        let mut modulator = qam::Modulator::new(16, 250, audio.clone());
        let mut demodulator = qam::Demodulator::new(16, 250, audio.clone());

        Physical {
            modulator: modulator,
            demodulator: demodulator,
            audio: audio
        }
    }

    pub fn send(&mut self, data: &str) {
        self.modulator.modulate(data);
    }

    pub fn recv(&mut self) -> String {
        String::from_str("Test")
    }
}

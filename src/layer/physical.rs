use qam;
use audio;

pub struct Physical {
    modulator: qam::Modulator,
    demodulator: qam::Demodulator,
    audio: audio::Audio
}

impl Physical {
    pub fn new() -> Physical {
        let mut audio = audio::Audio::new();

        let mut modulator = qam::Modulator::new(16, 250, audio.samp_rate);
        let mut demodulator = qam::Demodulator::new(16, 250, audio.samp_rate);

        Physical {
            modulator: modulator,
            demodulator: demodulator,
            audio: audio
        }
    }

    pub fn send(&mut self, data: &str) {
        let ref mut audio = self.audio;
        self.modulator.modulate(data, &mut |value: f32| { audio.write(value); });
    }

    pub fn recv(&mut self) -> String {
        String::from_str("Test")
    }
}

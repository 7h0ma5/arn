use qam;
use audio;

pub struct Physical {
    modulator: qam::Modulator,
}

impl Physical {
    pub fn new() -> Physical {
        let mut output = audio::Output::new();
        let mut modulator = qam::Modulator::new(16, 125, output);

        Physical {
            modulator: modulator
        }
    }

    pub fn send(&mut self, data: &str) {
        self.modulator.modulate(data);
    }

    pub fn recv(&mut self) -> String {
        String::from_str("Test")
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use std::f32::consts::PI;
use num::Complex;

use qam::Constellation;
use fir::Filter;
use audio::Audio;

pub struct Demodulator {
    constellation: Constellation,
    filter: Filter,
    audio: Rc<RefCell<Audio>>,
    baud_rate: usize,
    carrier: usize,
    time: usize
}

impl Demodulator {
    pub fn new(n: usize, baud_rate: usize, audio: Rc<RefCell<Audio>>) -> Demodulator {
        let filter = Filter::rrc(audio.borrow().samp_rate/baud_rate, 0.22);

        Demodulator {
            constellation: Constellation::new(n),
            filter: filter,
            audio: audio,
            baud_rate: baud_rate,
            carrier: 1500,
            time: 0
        }
    }

    pub fn demodulate(&mut self, data: &str) {

    }
}

use std::cell::RefCell;
use std::rc::Rc;
use std::f32::consts::PI;
use num::Complex;

use qam::Constellation;
use fir::Filter;
use audio::Audio;

pub struct Modulator {
    constellation: Constellation,
    filter: Filter,
    audio: Rc<RefCell<Audio>>,
    baud_rate: usize,
    carrier: usize,
    time: usize
}

impl Modulator {
    pub fn new(n: usize, baud_rate: usize, audio: Rc<RefCell<Audio>>) -> Modulator {
        let filter = Filter::rrc(audio.borrow().samp_rate/baud_rate, 0.22);
        let constellation = Constellation::new(n);
        println!("{:?}", constellation);

        Modulator {
            constellation: constellation,
            filter: filter,
            audio: audio,
            baud_rate: baud_rate,
            carrier: 1500,
            time: 0
        }
    }

    #[inline]
    pub fn modulate_symbol(&mut self, sym: usize) {
        let point = self.constellation.points[sym].scale(self.constellation.scale);
        let samples = self.audio.borrow().samp_rate/self.baud_rate;

        let w = 2.0 * PI * self.carrier as f32 / self.audio.borrow().samp_rate as f32;

        for _ in 0..samples {
            let value = self.filter.process(point);

            let t = self.time as f32;
            let phasor = Complex::from_polar(&0.4, &(w * t));
            let value = value * phasor;

            self.audio.borrow_mut().write(value.re);

            self.time = (self.time + 1) % self.audio.borrow().samp_rate;
        }
    }

    pub fn modulate(&mut self, data: &str) {
        let bits = self.constellation.bits_per_symbol;
        let mut size: usize = 0;
        let mut symbol: usize = 0;

        for byte in data.bytes() {
            for i in 0..8 {
                symbol |= (((byte as usize) & (1 << i)) >> i) << size;
                size += 1;

                if size >= bits {
                    self.modulate_symbol(symbol);
                    symbol = 0;
                    size = 0;
                }
            }
        }

        if size <= bits {
            self.modulate_symbol(symbol);
        }
    }
}

use std::f32::consts::PI;
use num::Complex;

use qam::Constellation;
use fir::Filter;
use audio::Audio;

pub struct Modulator {
    constellation: Constellation,
    filter: Filter,
    baud_rate: usize,
    samp_rate: usize,
    carrier: usize,
    time: usize
}

impl Modulator {
    pub fn new(n: usize, baud_rate: usize, samp_rate: usize) -> Modulator {
        let sps = samp_rate as f32 / baud_rate as f32;
        let nfilts = 32;
        let ntaps = nfilts * 11 * sps as usize;

        let taps = Filter::rrc(nfilts as f64, sps as f64, 0.22, ntaps);
        let constellation = Constellation::new(n);

        let filter = Filter::new(sps, taps, 32);

        Modulator {
            constellation: constellation,
            filter: filter,
            samp_rate: samp_rate,
            baud_rate: baud_rate,
            carrier: 1500,
            time: 0
        }
    }

    #[inline]
    pub fn modulate_symbol(&mut self, sym: usize, out: &mut Audio) {
        let point = self.constellation.points[sym];
        //let samples = self.samp_rate/self.baud_rate;

        //let w = 2.0 * PI * self.carrier as f32 / self.samp_rate as f32;

        let value = self.filter.process(point);

        //let t = self.time as f32;
        //let phasor = Complex::from_polar(&0.4, &(w * t));
        //let value = value * phasor;

        out.write(value.re);

        //self.time = (self.time + 1) % self.samp_rate;
    }

    pub fn modulate(&mut self, data: &str, mut out: &mut Audio) {
        let bits = self.constellation.bits_per_symbol;
        let mut size: usize = 0;
        let mut symbol: usize = 0;

        for byte in data.bytes() {
            for i in 0..8 {
                symbol |= (((byte as usize) & (1 << i)) >> i) << size;
                size += 1;

                if size >= bits {
                    self.modulate_symbol(symbol, out);
                    symbol = 0;
                    size = 0;
                }
            }
        }

        if size <= bits {
            self.modulate_symbol(symbol, out);
        }
    }
}

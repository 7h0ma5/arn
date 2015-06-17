use std::f32::consts::PI;
use complex::Complex;

use qam::Constellation;
use filter::{Filter, Resampler};
use audio::Audio;

pub struct Modulator {
    constellation: Constellation,
    resampler: Resampler,
    baud_rate: usize,
    samp_rate: usize,
    omega: f32,
    time: usize
}

impl Modulator {
    pub fn new(n: usize, baud_rate: usize, samp_rate: usize) -> Modulator {
        let sps = samp_rate as f32 / baud_rate as f32;
        let nfilts = 32;
        let ntaps = nfilts * 11 * sps as usize;

        let taps = Filter::rrc(nfilts as f64, nfilts as f64, 1.0, 0.22, ntaps);
        let constellation = Constellation::new(n);

        let resampler = Resampler::new(sps, taps, 32);
        println!("{:?}", resampler);

        let carrier = 1500;
        let omega = 2.0 * PI * carrier as f32 / samp_rate as f32;

        Modulator {
            constellation: constellation,
            resampler: resampler,
            samp_rate: samp_rate,
            baud_rate: baud_rate,
            omega: omega,
            time: 0
        }
    }

    #[inline]
    pub fn modulate_symbol(&mut self, sym: usize, out: &mut Audio) {
        let ref point = self.constellation.points[sym];
        let values = self.resampler.process(point);

        for value in values {
            let phasor = Complex::from_polar(0.4, self.omega * self.time as f32);
            let value = value * phasor;

            out.write(value.re);

            self.time += 1;
        }

        self.time = self.time % self.samp_rate;
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

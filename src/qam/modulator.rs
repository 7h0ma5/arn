use std::f32::consts::PI;
use std::io::{stdout, stderr};
use std::io::Write;
use num::Complex;

use qam::Constellation;
use fir::Filter;
use audio::Output;

pub struct Modulator {
    constellation: Constellation,
    filter: Filter,
    output: Output,
    baud_rate: usize,
    carrier: usize,
    time: usize
}

impl Modulator {
    pub fn new(n: usize, baud_rate: usize, output: Output) -> Modulator {
        let filter = Filter::rrc(output.samp_rate/baud_rate, 0.22);

        Modulator {
            constellation: Constellation::new(n),
            filter: filter,
            output: output,
            baud_rate: baud_rate,
            carrier: 1500,
            time: 0
        }
    }

    pub fn modulate_symbol(&mut self, sym: usize) {
        let mut out = stderr();

        let point = self.constellation.points[sym];
        let samples = self.output.samp_rate/self.baud_rate;

        for _ in 0..samples {
            let value = point.scale(1.0/self.constellation.max_amplitude);

            let value = self.filter.process(point).scale(0.4);

            let w = 2.0 * PI * self.carrier as f32;
            let t = self.time as f32/self.output.samp_rate as f32;
            let phasor = Complex::from_polar(&0.4, &(w * t));
            let value = value * phasor;

            self.output.write(value.re);

            //let sample = (phasor.re * 32767.0) as i16;
            //out.write_all(&[(sample >> 8) as u8, (sample & 0xff) as u8]);

            self.time = (self.time + 1) % self.output.samp_rate;
        }
    }

    pub fn modulate(&mut self, data: &str) {
        let bits = self.constellation.bits_per_symbol;
        let mut size: usize = 0;
        let mut symbol: usize = 0;

        let mut out = stdout();

        for byte in data.bytes() {
            //out.write_all(&[byte as u8]);
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

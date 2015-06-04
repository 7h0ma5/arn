use std::f32::consts::PI;
use std::io::stderr;
use std::io::Write;
use num::Complex;

use qam::Constellation;
use fir::Filter;

pub struct Demodulator {
    constellation: Constellation,
    filter: Filter,
    baud_rate: usize,
    samp_rate: usize,
    carrier: usize,
    time: usize
}

impl Demodulator {
    pub fn new(n: usize, baud_rate: usize, samp_rate: usize) -> Modulator {
        let filter = Filter::rrc(samp_rate/baud_rate, 0.22);

        Modulator {
            constellation: Constellation::new(n),
            filter: filter,
            samp_rate: samp_rate,
            baud_rate: baud_rate,
            carrier: 1500,
            time: 0
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

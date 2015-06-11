use num::Complex;

pub struct Filter {
    values: Vec<Complex<f32>>,
    taps: Vec<f32>,
    pos: usize
}

impl Filter {
    pub fn new(taps: Vec<f32>) -> Filter {
        let mut values = Vec::with_capacity(taps.len());

        for _ in 0..taps.len() {
            values.push(Complex::new(0.0, 0.0));
        }

        Filter {
            values: values,
            taps: taps,
            pos: 0
        }
    }

    #[inline]
    pub fn process(&mut self, value: Complex<f32>) -> Complex<f32> {
        let max = self.taps.len();
        let pos = self.pos;
        let mut out = Complex::new(0.0, 0.0);

        self.values[pos] = value;

        for i in 0..max {
            let idx = (pos + i) % max;
            out = out + self.values[idx].scale(self.taps[i]);
        }

        self.pos = (self.pos + 1) % max;

        out
    }
}

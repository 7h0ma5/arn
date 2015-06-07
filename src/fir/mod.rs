use num::Complex;

pub mod rrc;

pub struct Filter {
    values: Vec<Complex<f32>>,
    coeffs: Vec<f32>,
    scale: f32,
    pos: usize
}

impl Filter {
    pub fn new(coeffs: Vec<f32>) -> Filter {
        let mut values = Vec::with_capacity(coeffs.len());
        let mut gain = 0.0;

        for coeff in coeffs.iter() {
            gain += *coeff;
            values.push(Complex::new(0.0, 0.0));
        }

        Filter {
            values: values,
            coeffs: coeffs,
            scale: 1.0/gain,
            pos: 0
        }
    }

    #[inline]
    pub fn process(&mut self, value: Complex<f32>) -> Complex<f32> {
        let max = self.coeffs.len();
        let pos = self.pos;
        let mut out = Complex::new(0.0, 0.0);

        self.values[pos] = value;

        for i in 0..max {
            let idx = (pos + i) % max;
            out = out + self.values[idx].scale(self.coeffs[i]);
        }

        self.pos = (self.pos + 1) % max;

        return out.scale(self.scale);
    }
}

use std::collections::vec_deque::VecDeque;
use num::Complex;

pub mod rrc;

pub struct Filter {
    values: VecDeque<Complex<f32>>,
    coeffs: Vec<f32>,
    gain: f32
}

impl Filter {
    pub fn new(coeffs: Vec<f32>) -> Filter {
        let mut values = VecDeque::with_capacity(coeffs.len());
        let mut gain = 0.0;

        for coeff in coeffs.iter() {
            gain += *coeff;
            values.push_back(Complex::new(0.0, 0.0));
        }

        Filter {
            values: values,
            coeffs: coeffs,
            gain: gain
        }
    }

    pub fn process(&mut self, value: Complex<f32>) -> Complex<f32> {
        let max = self.coeffs.len();
        let mut out = Complex::new(0.0, 0.0);

        self.values.pop_back();
        self.values.push_front(value);

        for i in 0..max {
            out = out + self.values[i].scale(self.coeffs[i]);
        }

        return out.scale(1.0 / self.gain);
    }
}

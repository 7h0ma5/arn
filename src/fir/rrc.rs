use std::collections::vec_deque::VecDeque;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct RootRaisedCosine {
    values: VecDeque<f32>,
    coeffs: Vec<f32>
}

impl RootRaisedCosine {
    pub fn new(n: usize, r: f32) -> RootRaisedCosine {
        let osf = 2;
        let mut coeffs = Vec::with_capacity(n);
        let mut values = VecDeque::with_capacity(n);

        for i in 0..(osf*n+1) {
            let t = (n as isize - i as isize) as f32 / osf as f32;

            let a = (PI * t * (1.0 - r)).sin();
            let b = 4.0 * r * t * (PI * t * (1.00 + r)).cos();
            let c = PI * t * (1.0 - (4.0 * r * t).powf(2.0));
            let d = (a+b)/c;

            let v = if !d.is_nan() { d } else { 0.0 };
            coeffs.push(v);
            values.push_back(0.0);
        }

        RootRaisedCosine {
            values: values,
            coeffs: coeffs
        }
    }

    pub fn process(&mut self, value: f32) -> f32 {
        let max = self.coeffs.len();
        let mut out = 0.0;

        self.values.pop_back();
        self.values.push_front(value);

        for i in 0..max {
            out += self.coeffs[i] * self.values[i];
        }

        return out;
    }
}

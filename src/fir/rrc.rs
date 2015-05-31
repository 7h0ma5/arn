use std::collections::vec_deque::VecDeque;
use std::f32::consts::PI;
use num::Complex;

#[derive(Debug)]
pub struct RootRaisedCosine {
    values: VecDeque<Complex<f32>>,
    coeffs: Vec<f32>,
    gain: f32
}

impl RootRaisedCosine {
    pub fn new(n: usize, r: f32) -> RootRaisedCosine {
        let mut coeffs = Vec::with_capacity(n);
        let mut values = VecDeque::with_capacity(n);
        let mut gain = 0.0;

        for i in 0..(n+1) {
            let t = (n as isize - i as isize) as f32;

            let a = (PI * t * (1.0 - r)).sin();
            let b = 4.0 * r * t * (PI * t * (1.00 + r)).cos();
            let c = PI * t * (1.0 - (4.0 * r * t).powf(2.0));
            let mut d = (a+b)/c;

            if d.is_nan() && ((t - 0.0).abs() < 0.1) {
                d = (1.0 - r) + (4.0 * r) / PI;
            }
            else if d.is_nan() {
                d = 0.0;
                panic!("singularity");
            }

            //println!("{}\t{}", i, d);
            gain += d;
            coeffs.push(d);
            values.push_back(Complex::new(0.0, 0.0));
        }

        RootRaisedCosine {
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

        return out.scale(self.gain * 0.9);
    }

}

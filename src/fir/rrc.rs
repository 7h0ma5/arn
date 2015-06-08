use fir::Filter;
use std::f32::consts::PI;
use num::Complex;

impl Filter {
    pub fn rrc(s: usize, r: f32) -> Filter {
        let n = 2 * s;
        let mut coeffs = Vec::with_capacity(n+1);
        let s = s as f32;

        for i in 0..(n+1) {
            let t = i as f32 - (n as f32/2.0);

            if t == 0.0 {
                let d = 1.0 / s.sqrt() * (1.0 - r + 4.0 * r/PI);
                coeffs.push(d);
            }
            else if (t.abs() - s/(4.0*r)).abs() < 0.1 {
                let a = (1.0 + 2.0/PI) * (PI/(4.0*r)).sin();
                let b = (1.0 - 2.0/PI) * (PI/(4.0*r)).cos();
                let d = r / (2.0 * s).sqrt() * (a + b);
                coeffs.push(d);
            }
            else {
                let a = (PI * t/s * (1.0 - r)).sin();
                let b = 4.0 * r * t/s * (PI * t/s * (1.0 + r)).cos();
                let c = PI * t/s * (1.0 - (4.0 * r * t/s).powf(2.0));
                let d = 1.0 / s.sqrt() * ((a + b) / c);
                coeffs.push(d);
            }
        }

        Filter::new(coeffs)
    }
}

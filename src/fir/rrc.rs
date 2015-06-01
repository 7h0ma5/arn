use fir::Filter;
use std::f32::consts::PI;
use num::Complex;

impl Filter {
    pub fn rrc(n: usize, r: f32) -> Filter {
        let mut coeffs = Vec::with_capacity(n);

        for i in 0..(n+1) {
            let t = (i as isize - (n/2) as isize) as f32 / n as f32;

            let a = (PI * t * (1.0 - r)).sin();
            let b = 4.0 * r * t * (PI * t * (1.00 + r)).cos();
            let c = PI * t * (1.0 - (4.0 * r * t).powf(2.0));
            let mut d = (a+b)/c;

            if d.is_nan() && ((t - 0.0).abs() < 0.1) {
                d = (1.0 - r) + (4.0 * r) / PI;
            }
            else if d.is_infinite() {
                let e = (1.0 + 2.0/PI) * (PI / (4.0*r)).sin();
                let f = (1.0 - 2.0/PI) * (PI / (4.0*r)).cos();
                d = r/(2.0 as f32).sqrt() * (e + f);
            }
            else if !d.is_finite() {
                d = 0.0;
                panic!("singularity");
            }

            //println!("{}\t{}", t, d);

            coeffs.push(d);
        }

        Filter::new(coeffs)
    }
}

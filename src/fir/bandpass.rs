use std::collections::vec_deque::VecDeque;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Bandpass {
    values: VecDeque<f32>,
    coeffs: Vec<f32>
}

impl Bandpass {
    pub fn new(center: f32, bandwidth: f32, att: f32, samp_rate: usize) -> Bandpass {
        println!("center: {}, bandwidth: {}, att: {}, samp_rate: {}", center, bandwidth, att, samp_rate);
        let a = window_shape(att);
        let a_bessel0 = bessel0(a);

        let fa: f32 = center-(bandwidth/2.0);
        let fb: f32 = center+(bandwidth/2.0);
        let fs = samp_rate as f32;

        let m1 = ((att - 8.0) / (14.36 * bandwidth/fs)) as usize;
        let m = if (m1 % 2 == 0) { m1 + 1 } else { m1 + 2 };
        println!("m: {}", m);
        let np = (m-1)/2;

        let mut coeffs = vec![0.0; m];
        let mut values = VecDeque::with_capacity(m);
        for _ in 0..m { values.push_back(0.0); }

        coeffs[0] = 2.0*bandwidth/fs;

        for i in 1..(np+1) {
            let j = i as f32;
            let x = (2.0*PI*j*fb/fs).sin();
            let y = (2.0*PI*j*fa/fs).sin();
            coeffs[i] = (x - y) / (PI*j);
        }

        for i in 0..(np+1) {
            let j = i as f32;
            let x = coeffs[i];
            let y = (1.0 - ((j*j) / (np*np) as f32)).sqrt();
            coeffs[np+i] = x*bessel0(a*y) / a_bessel0;
        }

        for i in 0..np {
            let v = coeffs[m-1-i];
            coeffs[i] = v;
        }

        Bandpass {
            coeffs: coeffs,
            values: values
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

fn window_shape(att: f32) -> f32 {
    if (att > 50.0) {
        0.1102 * (att - 8.7)
    }
    else if (att >= 21.0) {
        0.5842 * (att - 21.0).powf(0.4) + 0.07886 * (att - 21.0)
    }
    else {
        0.0
    }
}

fn bessel0(x: f32) -> f32 {
    let mut d: f32 = 0.0;
    let mut ds: f32 = 1.0;
    let mut s: f32 = 1.0;

    loop {
        d += 2.0;
        ds *= (x*x)/(d*d);
        s += ds;
        if (ds.is_nan() || ds <= s * 1.0e-6) { break };
    }

    return s;
}

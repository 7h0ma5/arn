use filter::Filter;
use std::f64::consts::PI;

impl Filter {
    /// Generate root raised cosine filter taps.
    pub fn rrc(gain: f64, samp_rate: f64, baud_rate: f64, alpha: f64, ntaps: usize) -> Vec<f32> {
        let ntaps = ntaps | 1; // make ntaps odd
        let sps = samp_rate/baud_rate;
        let mut taps: Vec<f64> = Vec::with_capacity(ntaps);
        let mut scale: f64 = 0.0;

        for i in 0..ntaps {
            let t = i as f64 - (ntaps as f64/2.0);
            let mut num: f64;
            let mut den: f64;

            let mut x1 = PI*t/sps;
            let mut x2 = 4.0*alpha*t/sps;
            let mut x3 = x2*x2 - 1.0;

            if x3.abs() >= 0.000001 {
                let x4 = ((1.0 + alpha)*x1).cos();
                den = x3 * PI;
                if (i != ntaps/2) {
                    num = x4 + ((1.0 - alpha)*x1).sin()/(4.0*alpha*t/sps);
                }
                else {
                    num = x4 + (1.0 - alpha)*PI/(4.0*alpha);
                }
            }
            else {
                if alpha == 1.0 {
                    taps.push(-1.0);
                    scale += -1.0;
                    continue;
                }
                x3 = (1.0 - alpha)*x1;
                x2 = (1.0 + alpha)*x1;
                num = x2.sin()*(1.0 + alpha)*PI
                      - x3.cos()*((1.0 - alpha)*PI*sps)/(4.0*alpha*t)
                      + x3.sin()*sps*sps/(4.0*alpha*t*t);
                den = -32.0*PI*alpha*alpha*t/sps;
            }

            taps.push(4.0*alpha*num/den);
            scale += taps[i];
        }

        let mut out = Vec::with_capacity(ntaps);

        for tap in taps.iter() {
            let value = tap * gain / scale;
            println!("{}", value);
            out.push(value as f32);
        }

        //println!("{} taps", out.len());

        out
    }
}

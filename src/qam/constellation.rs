use num::pow;
use num::Complex;
use std::cmp::max;

pub struct Constellation {
    pub points: Vec<Complex<f32>>,
    pub avg_power: f32,
    pub bits_per_symbol: usize,
    pub max_amplitude: f32
}

impl Constellation {
    pub fn new(n: usize) -> Constellation {
        let bits = (n as f32).log2() as usize;
        let symbols = pow(2, bits);

        let rows = pow(2, bits - bits/2) as isize;
        let columns = pow(2, bits/2) as isize;

        let mut points = vec![Complex::new(0.0, 0.0); symbols];
        let mut sum = 0.0;
        let mut max_amplitude = 0.0f32;

        for y in 0..rows {
            let imaginary = (2 * y - (rows - 1)) as f32;
            for x in 0..columns {
                let real = (2 * x - (columns - 1)) as f32;
                let point = Complex::new(real, imaginary);
                let norm = point.norm();

                let symbol = (y * columns + x) as usize;
                points[symbol] = point;
                sum += norm;

                if (norm > max_amplitude) {
                    max_amplitude = norm;
                }
            }
        }

        Constellation {
            points: points,
            avg_power: sum/(bits as f32)/max_amplitude,
            bits_per_symbol: bits,
            max_amplitude: max_amplitude
        }
    }
}

use filter::Filter;
use num::Complex;

pub struct Resampler {
    filters: Vec<Filter>,
    rate: f32
}

impl Resampler {
    pub fn new(rate: f32, taps: Vec<f32>, filter_size: usize) -> Resampler {
        let taps_per_filter = (taps.len() as f32 / filter_size as f32).ceil() as usize;
        let last_filter = (taps.len()/2) % filter_size;

        let mut filters = Vec::with_capacity(filter_size);

        for i in 0..filter_size {
            let mut filter_taps = Vec::with_capacity(taps_per_filter);

            for i in ((0 + i)..taps.len()).step_by(filter_size) {
                filter_taps.push(taps[i]);
            }

            while filter_taps.len() < taps_per_filter {
                filter_taps.push(0.0);
            }

            filters.push(Filter::new(filter_taps));
        }

        Resampler {
            filters: filters,
            rate: rate
        }
    }

    #[inline]
    pub fn process(&mut self, value: Complex<f32>) -> Complex<f32> {
        Complex::new(0.0, 0.0)
    }
}

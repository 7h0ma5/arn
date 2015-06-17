use filter::Filter;
use complex::Complex;

#[derive(Debug)]
pub struct Resampler {
    filters: Vec<Filter>,
    diff_filters: Vec<Filter>,
    flt_rate: f32,
    int_rate: usize,
    dec_rate: usize,
    last_filter: usize,
    taps_per_filter: usize,
    acc: f32
}

impl Resampler {
    pub fn new(rate: f32, taps: Vec<f32>, filter_size: usize) -> Resampler {
        let taps_per_filter = (taps.len() as f32 / filter_size as f32).ceil() as usize;
        let last_filter = (taps.len()/2) % filter_size;

        let int_rate = filter_size;
        let dec_rate = (int_rate as f32 / rate).floor() as usize;
        let flt_rate = (int_rate as f32 / rate) - dec_rate as f32;

        let mut filters = Vec::with_capacity(filter_size);
        let mut diff_taps = Vec::with_capacity(taps.len());
        let mut diff_filters = Vec::with_capacity(filter_size);

        let mut diff_filter = Vec::with_capacity(2);
        diff_filter.push(-1.0);
        diff_filter.push(1.0);

        for i in 0..taps.len()-1 {
            let mut tap = 0.0;
            for j in 0..diff_filter.len() {
                tap += diff_filter[j] * taps[i+j];
            }
            diff_taps.push(tap);
        }

        diff_taps.push(0.0);

        for i in 0..filter_size {
            let mut filter_taps = Vec::with_capacity(taps_per_filter);
            let mut diff_filter_taps = Vec::with_capacity(taps_per_filter);

            for i in ((0 + i)..taps.len()).step_by(filter_size) {
                filter_taps.push(taps[i]);
                diff_filter_taps.push(diff_taps[i]);
            }

            while filter_taps.len() < taps_per_filter {
                filter_taps.push(0.0);
                diff_filter_taps.push(0.0);
            }

            filters.push(Filter::new(filter_taps));
            diff_filters.push(Filter::new(diff_filter_taps));
        }

        Resampler {
            filters: filters,
            diff_filters: diff_filters,
            int_rate: int_rate,
            dec_rate: dec_rate,
            flt_rate: flt_rate,
            taps_per_filter: taps_per_filter,
            last_filter: last_filter,
            acc: 0.0
        }
    }

    #[inline]
    pub fn process(&mut self, value: &Complex) -> Vec<Complex> {
        let mut out = Vec::with_capacity(self.int_rate);
        let mut i = self.last_filter;

        while i < self.int_rate {
            let o0 = self.filters[i].process(value);
            let o1 = self.diff_filters[i].process(value);

            out.push(o0 + o1.scale_new(self.acc));

            self.acc += self.flt_rate;
            i += self.dec_rate + self.acc.floor() as usize;
            self.acc = self.acc % 1.0;
        }

        self.last_filter = i % self.int_rate;

        out
    }
}

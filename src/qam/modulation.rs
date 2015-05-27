use num::complex::Complex;

pub type Constellation = Vec<Complex<usize, usize>>;

impl Constellation {
    pub fn new(n: usize) -> Constellation {
        let mut constellation = vec![(0, 0); n];

        let columns = num::pow(2, n/2);
        let rows = num::pow(2, n - n/2);
    }

    pub fn bits_per_symbol(&self) -> usize {
        (self.length as f32).log2() as usize
    }

    pub fn make_constellation(&self) -> Constellation {

        for p in 0..self.phases {
            for a in 0..self.amplitudes {
                let s = p * self.amplitudes + a;
                constellation[s] = (self.phases - p - 1, a + 1);
            }
        }
        constellation
    }
}

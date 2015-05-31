#![feature(step_by, convert, collections)]

extern crate portaudio;
extern crate num;

use std::io::prelude::*;
use std::fs::File;
use num::Complex;

mod audio;
mod fir;
mod qam;

fn main() {
    //audio::init();

    /*
    let mut modulator = qam::Modulator::new(4, 500, 44100);

    let mut f = File::open("/usr/share/licenses/common/GPL3/license.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    modulator.modulate(s.slice_chars(0, 500));
    */

    let mut test = fir::RootRaisedCosine::new(100, 0.22);

    for i in 0..100 {
        println!("{}\t{}\t{}", i, 1.0, test.process(Complex::new(1.0, 0.0)).re);
    }

    for i in 0..100 {
        println!("{}\t{}\t{}", i+100, 1.0, test.process(Complex::new(1.0, 0.0)).re);
    }

    for i in 0..100 {
        println!("{}\t{}\t{}", i+200, 0.5, test.process(Complex::new(0.5, 0.0)).re);
    }

    for i in 0..100 {
        println!("{}\t{}\t{}", i+300, -1.0, test.process(Complex::new(-1.0, 0.0)).re);
    }

    for i in 0..100 {
        println!("{}\t{}\t{}", i+400, -1.0, test.process(Complex::new(-1.0, 0.0)).re);
    }

    for i in 0..100 {
        println!("{}\t{}\t{}", i+500, 1.0, test.process(Complex::new(1.0, 0.0)).re);
    }
}

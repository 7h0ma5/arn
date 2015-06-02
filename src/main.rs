#![feature(step_by, convert, collections)]

extern crate portaudio;
extern crate num;

use std::thread;
use std::io::prelude::*;
use std::fs::File;
use num::Complex;

mod layer;
mod audio;
mod fir;
mod qam;

fn main() {
    //audio::init();

    let mut f = File::open("/usr/share/licenses/common/GPL3/license.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    //modulator.modulate(s.slice_chars(0, 500));

    let mut physical = layer::Physical::new();

    let input = physical.tx.clone();

    thread::spawn(move || {
        physical.run();
    });

    input.send(s.as_str());

    loop {}

    /*
    let n = 8;

    let mut test = fir::Filter::rrc(n, 0.5);


    for i in 0..n {
        println!("{}\t{}\t{}", i, 1.0, test.process(Complex::new(1.0, 0.0)).re);
    }

    for i in 0..n {
        println!("{}\t{}\t{}", i+n, 1.0, test.process(Complex::new(1.0, 0.0)).re);
    }

    for i in 0..n {
        println!("{}\t{}\t{}", i+2n, 0.5, test.process(Complex::new(0.5, 0.0)).re);
    }

    for i in 0..n {
        println!("{}\t{}\t{}", i+3n, -1.0, test.process(Complex::new(-1.0, 0.0)).re);
    }

    for i in 0..n {
        println!("{}\t{}\t{}", i+4n, -1.0, test.process(Complex::new(-1.0, 0.0)).re);
    }

    for i in 0..n {
        println!("{}\t{}\t{}", i+5n, 1.0, test.process(Complex::new(1.0, 0.0)).re);
    }
    */
}

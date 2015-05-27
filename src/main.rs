#![feature(step_by, convert, collections)]

extern crate portaudio;
extern crate num;

use std::io::prelude::*;
use std::fs::File;

mod audio;
mod fir;
mod qam;

fn main() {
    //audio::init();
    let mut modulator = qam::Modulator::new(4, 31, 44100);

    let mut f = File::open("/usr/share/licenses/common/GPL3/license.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    modulator.modulate(s.slice_chars(0, 1000));
}

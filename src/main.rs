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
    audio::init();

    //let mut input = audio::Input::new();

    //let mut f = File::open("/usr/share/licenses/common/GPL3/license.txt").unwrap();
    let mut f = File::open("/tmp/test").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let mut physical = layer::Physical::new();

    physical.send(s.as_str());

    /*
    let input = physical.tx.clone();

    thread::spawn(move || {
        physical.run();
    });

    input.send(s);

    loop {}
    */
}

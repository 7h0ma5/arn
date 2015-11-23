#![feature(step_by, convert, collections)]

extern crate portaudio;
extern crate num;

use std::thread;
use std::io::prelude::*;
use std::fs::File;

mod layer;
mod audio;
mod complex;
mod filter;
mod qam;

fn main() {
    //let mut f = File::open("/usr/share/licenses/common/GPL3/license.txt").unwrap();
    let mut f = File::open("/tmp/test.dat").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let mut physical = layer::Physical::new();

    //physical.send(s.slice_chars(0, 1000));
    physical.send(s.as_str());
}

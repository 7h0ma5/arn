use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use qam;

pub struct Physical {
    modulator: qam::Modulator,
    rx: Receiver<&str>,
    pub tx: Sender<&str>,
}

impl Physical {
    pub fn new() -> Physical {
        let mut modulator = qam::Modulator::new(4, 125, 44100);
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        Physical {
            modulator: modulator,
            rx: rx,
            tx: tx
        }
    }

    pub fn run(&mut self) {
        println!("running physical layer!");
        loop {
            let data = self.rx.recv().unwrap();
            println!("got data: {}", data);
        }
    }
}

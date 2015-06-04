use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use qam;

pub struct Physical {
    modulator: qam::Modulator,
    rx: Receiver<String>,
    pub tx: Sender<String>,
}

impl Physical {
    pub fn new() -> Physical {
        let mut modulator = qam::Modulator::new(4, 125, 44100);
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

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

use qam;

pub struct Physical {
    modulator: qam::Modulator,
}

impl Physical {
    pub fn new() -> Physical {
        let mut modulator = qam::Modulator::new(4, 125, 44100);

        Physical {
            modulator: modulator
        }
    }

    pub fn run(&mut self) {
        println!("running physical layer!");
        loop {
            let data = self.rx.recv().unwrap();
            println!("got data: {}", data);
        }
    }

    pub fn send(&mut self, String data) {

    }

    pub fn recv(&mut self) -> String {

    }
}

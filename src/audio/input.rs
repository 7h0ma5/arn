use portaudio::pa;

pub struct Input {
    stream: pa::Stream<f32, f32>
}

impl Input {
    pub fn new() -> Input {
        let mut stream = pa::Stream::new();
        stream.open_default(44100.0, 256, 1, 0, pa::SampleFormat::Float32, None).unwrap();
        stream.start();

        Input {
            stream: stream
        }
    }

    pub fn read(&mut self) -> Vec<f32> {
        self.stream.read(100).unwrap()
    }
}

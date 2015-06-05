use portaudio::pa;

const FRAMES_PER_BUFFER: usize = 1024;
const SAMPLE_RATE: usize = 44100;

pub struct Output {
    stream: pa::Stream<f32, f32>,
    buffer: Vec<f32>,
    pub samp_rate: usize
}

impl Output {
    pub fn new() -> Output {
        let mut stream = pa::Stream::new();

        stream.open_default(SAMPLE_RATE as f64, FRAMES_PER_BUFFER as u32, 0, 1, pa::SampleFormat::Float32, None).unwrap();
        stream.start().unwrap();

        Output {
            stream: stream,
            buffer: Vec::with_capacity(FRAMES_PER_BUFFER),
            samp_rate: SAMPLE_RATE
        }
    }

    #[inline]
    pub fn write(&mut self, value: f32)  {
        if (self.buffer.len() >= FRAMES_PER_BUFFER) {
            println!("write buffer!");
            self.stream.write(self.buffer.clone(), FRAMES_PER_BUFFER as u32).unwrap();
            self.buffer.clear();
        }

        self.buffer.push(value);
    }
}

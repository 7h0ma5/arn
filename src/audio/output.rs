use portaudio::pa;
use std::mem::replace;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

const FRAMES_PER_BUFFER: usize = 16384;
const SAMPLE_RATE: usize = 44100;

pub struct Output {
    stream: pa::Stream<f32, f32>,
    buffer: Vec<f32>,
    tx: SyncSender<Vec<f32>>,
    pub samp_rate: usize
}

impl Output {
    pub fn new() -> Output {
        let mut stream = pa::Stream::new();
        let (tx, rx) = sync_channel::<Vec<f32>>(10);

        let callback = Box::new(move |
            input: &[f32], output: &mut[f32], frames: u32, time_info: &pa::StreamCallbackTimeInfo,
            _flags: pa::StreamCallbackFlags,
        | -> pa::StreamCallbackResult {

            let packet = rx.recv();
            if packet.is_err() { return pa::StreamCallbackResult::Abort; }

            let frames = packet.unwrap();

            for (sample, output) in frames.iter().zip(output.iter_mut()) {
                *output = *sample;
            }

            pa::StreamCallbackResult::Continue
        });

        stream.open_default(SAMPLE_RATE as f64, FRAMES_PER_BUFFER as u32, 0, 1,
                            pa::SampleFormat::Float32, Some(callback)).unwrap();

        stream.start().unwrap();

        Output {
            stream: stream,
            buffer: Vec::with_capacity(FRAMES_PER_BUFFER),
            samp_rate: SAMPLE_RATE,
            tx: tx
        }
    }

    #[inline]
    pub fn write(&mut self, value: f32)  {
        if (self.buffer.len() >= FRAMES_PER_BUFFER) {
            let buffer = replace(&mut self.buffer, Vec::with_capacity(FRAMES_PER_BUFFER));
            self.tx.send(buffer).unwrap();
        }

        self.buffer.push(value);
    }
}

use std::mem::replace;
use std::error::Error;
use portaudio::pa;

const CHANNELS: u32 = 1;
const FRAMES: u32 = 128;

pub fn init() {
    pa::initialize().unwrap();

    let def_input = pa::device::get_default_input();
    let input_info = pa::device::get_info(def_input).unwrap();


    let input_stream_params = pa::StreamParameters {
        device: def_input,
        channel_count: CHANNELS as i32,
        sample_format: pa::SampleFormat::Float32,
        suggested_latency: input_info.default_low_input_latency
    };

    let def_output = pa::device::get_default_output();
    let output_info = pa::device::get_info(def_output).unwrap();

    let output_stream_params = pa::StreamParameters {
        device : def_output,
        channel_count : CHANNELS as i32,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : output_info.default_low_output_latency
    };

    let mut stream: pa::Stream<f32, f32> = pa::Stream::new();

    stream.open(Some(&input_stream_params), Some(&output_stream_params),
                input_info.default_sample_rate, FRAMES,
                pa::StreamFlags::empty(), None).unwrap();

    stream.start().unwrap();

    fn wait_for_stream<F: Fn() -> Result<pa::StreamAvailable, pa::error::Error>>(f: F, name: &str)
                                                                                     -> u32 {
        'waiting_for_stream: loop {
            match f() {
                Ok(available) => match available {
                    pa::StreamAvailable::Frames(frames) => return frames as u32,
                    pa::StreamAvailable::InputOverflowed => println!("Input stream has overflowed"),
                    pa::StreamAvailable::OutputUnderflowed => println!("Output stream has underflowed"),
                },
                Err(err) => panic!("An error occurred while waiting for the {} stream: {}", name, err.description()),
            }
        }
    };

    let mut buffer = Vec::with_capacity((FRAMES * CHANNELS) as usize);

    'stream: loop {
        let in_frames = wait_for_stream(|| stream.get_stream_read_available(), "Read");

        if in_frames > 0 {
            match stream.read(in_frames) {
                Ok(input_samples) => {
                    buffer.extend(input_samples.into_iter());
                    println!("Read {:?} frames from the input stream.", in_frames);
                },
                Err(err) => {
                    println!("An error occurred while reading from the input stream: {}", err.description());
                    break 'stream
                },
            }
        }

        let out_frames = wait_for_stream(|| stream.get_stream_write_available(), "Write");

        let buffer_frames = (buffer.len() / CHANNELS as usize) as u32;

        if out_frames > 0 && buffer_frames > 0 {

            let (write_buffer, write_frames) = if buffer_frames >= out_frames {
                let out_samples = (out_frames * CHANNELS as u32) as usize;
                let remaining_buffer = buffer[out_samples..].iter().map(|&sample| sample).collect();
                buffer.truncate(out_samples);
                let write_buffer = replace(&mut buffer, remaining_buffer);
                (write_buffer, out_frames)
            }

            else {
                let write_buffer = replace(&mut buffer, Vec::with_capacity((FRAMES * CHANNELS) as usize));
                (write_buffer, buffer_frames)
            };
            match stream.write(write_buffer, write_frames) {
                Ok(_) => println!("Wrote {:?} frames to the output stream.", out_frames),
                Err(err) => {
                    println!("An error occurred while writing to the output stream: {}", err.description());
                    break 'stream
                },
            }
        }
        'stream: loop {

            // How many frames are available on the input stream?
            let in_frames = wait_for_stream(|| stream.get_stream_read_available(), "Read");

            // If there are frames available, let's take them and add them to our buffer.
            if in_frames > 0 {
                match stream.read(in_frames) {
                    Ok(input_samples) => {
                        buffer.extend(input_samples.into_iter());
                        println!("Read {:?} frames from the input stream.", in_frames);
                    },
                    Err(err) => {
                        println!("An error occurred while reading from the input stream: {}", err.description());
                        break 'stream
                    },
                }
            }

            // How many frames are available for writing on the output stream?
            let out_frames = wait_for_stream(|| stream.get_stream_write_available(), "Write");

            // How many frames do we have so far?
            let buffer_frames = (buffer.len() / CHANNELS as usize) as u32;

            // If there are frames available for writing and we have some to write, then write!
            if out_frames > 0 && buffer_frames > 0 {
                // If we have more than enough frames for writing, take them from the start of the buffer.
                let (write_buffer, write_frames) = if buffer_frames >= out_frames {
                    let out_samples = (out_frames * CHANNELS as u32) as usize;
                    let remaining_buffer = buffer[out_samples..].iter().map(|&sample| sample).collect();
                    buffer.truncate(out_samples);
                    let write_buffer = replace(&mut buffer, remaining_buffer);
                    (write_buffer, out_frames)
                }
                // Otherwise if we have less, just take what we can for now.
                else {
                    let write_buffer = replace(&mut buffer, Vec::with_capacity((FRAMES * CHANNELS) as usize));
                    (write_buffer, buffer_frames)
                };
                match stream.write(write_buffer, write_frames) {
                    Ok(_) => println!("Wrote {:?} frames to the output stream.", out_frames),
                    Err(err) => {
                        println!("An error occurred while writing to the output stream: {}", err.description());
                        break 'stream
                    },
                }
            }

        }

        match stream.close() {
            Ok(()) => println!("Successfully closed the stream."),
            Err(err) => println!("An error occurred while closing the stream: {}", err.description()),
        }

        println!("");

        match pa::terminate() {
            Ok(()) => println!("Successfully terminated PortAudio."),
            Err(err) => println!("An error occurred while terminating PortAudio: {}", err.description()),
                }
    }

    stream.close().unwrap();
    pa::terminate().unwrap();
}

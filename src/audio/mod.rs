use portaudio::pa;

mod input;
pub use self::input::Input;

mod output;
pub use self::output::Output;

pub fn init() {
    pa::initialize().unwrap();
}

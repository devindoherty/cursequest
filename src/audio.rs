use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

pub struct Audio {
    pub sink: Sink,
    pub stream: OutputStream,
}

impl Audio {
    fn new(sink: Sink, stream: OutputStream) -> Audio {
        Audio {
            sink,
            stream,
        }
    }
}


pub fn play_audio() -> bool {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("assets/title_theme.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    sink.append(source);
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    sink.sleep_until_end();

    true
}
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rodio::source::{SineWave, Source};

pub struct MusicManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    pub sink: Sink,
    pub playing: bool,
}

impl MusicManager {
    pub fn new() -> MusicManager {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        MusicManager {
            _stream,
            stream_handle,
            sink,
            playing: true,
        }
    }
}

pub struct Audio {
    pub name: String,
    pub sound: Decoder<BufReader<File>>,
}

impl Audio {
    pub fn new(name: String, path: &str) -> Audio {
        let file = BufReader::new(File::open(path).unwrap());
        let sound = Decoder::new(file).unwrap();

        Audio {
            name,
            sound,
        }
    }
}
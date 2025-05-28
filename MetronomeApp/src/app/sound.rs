use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn play_sound(path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.detach(); // keeps playing in the background
}

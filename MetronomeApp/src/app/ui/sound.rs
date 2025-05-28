use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

use crate::app::Sounds;

pub fn play_metronome(sound: Sounds) {
    let s = sound.to_string().to_lowercase();
    let path = format!("assets/sounds/{}.wav", s);
    play_sound(path.as_str());
    println!("Printing {}", path.as_str());
}

fn play_sound(path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file: File = File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end(); // keeps playing in the background
}

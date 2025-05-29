use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

use crate::app::MyApp;
use crate::app::Sounds;

pub fn play_metronome(app: &mut MyApp, sound: Sounds) {
    let s = sound.to_string().to_lowercase();
    let path = format!("assets/sounds/{}.wav", s);

    app.audio = Some(play_sound(path.as_str(), app.volume));
}

fn play_sound(path: &str, volume: f32) -> (OutputStream, Sink) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(volume); // Should be between 0 and 1

    let file = File::open(&path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    (stream, sink) // ✅ keep stream alive by returning it
}

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

use crate::app::AppData;

pub fn play_metronome(app: &mut AppData, sound: String) {
    let path = format!("assets/sounds/{}.wav", sound);

    println!("Playing Sound: {}", path.as_str());
    app.runtime.audio = Some(play_sound(path.as_str(), app.parameters.volume));
}

fn play_sound(path: &str, volume: f32) -> (OutputStream, Sink) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(volume); // Should be between 0 and 1

    let file = File::open(&path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    (stream, sink)
}

use crate::app::systems::audio::sine_wave::SineWave;
use once_cell::sync::Lazy;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::{
    fs::File,
    io::BufReader,
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

pub enum Cmd {
    Sine { freq: f32, dur_ms: u64, amp: f32 },
    File { path: String, volume: f32 },
    Cleanup,
}

pub struct Audio {
    pub tx: Sender<Cmd>,
}

pub static AUDIO: Lazy<Audio> = Lazy::new(|| {
    let (tx, rx) = channel::<Cmd>();

    thread::spawn(move || {
        // `OutputStream` never leaves this thread:
        let (_stream, handle): (OutputStream, OutputStreamHandle) =
            OutputStream::try_default().unwrap();

        let mut sinks: Vec<Sink> = Vec::new();

        while let Ok(cmd) = rx.recv() {
            match cmd {
                Cmd::Sine { freq, dur_ms, amp } => {
                    let sink = Sink::try_new(&handle).unwrap();
                    let src = SineWave::new(freq, amp)
                        .take_duration(Duration::from_millis(dur_ms))
                        .fade_in(Duration::from_millis(5))
                        .fade_out(Duration::from_millis(dur_ms.saturating_sub(1)));
                    sink.append(src);
                    sinks.push(sink);
                }
                Cmd::File { path, volume } => {
                    let sink = Sink::try_new(&handle).unwrap();
                    sink.set_volume(volume);
                    let file = File::open(path).unwrap();
                    let src = Decoder::new(BufReader::new(file))
                        .unwrap()
                        .fade_in(Duration::from_millis(1));
                    sink.append(src);
                    sinks.push(sink);
                }
                Cmd::Cleanup => {
                    sinks.retain(|s| !s.empty());
                }
            }
        }
    });

    Audio { tx }
});

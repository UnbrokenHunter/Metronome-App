use once_cell::sync::Lazy;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::mpsc::{Sender, channel},
    thread,
    time::Duration,
};

pub enum Cmd {
    File { path: PathBuf, volume: f32 },
    Cleanup,
}

pub struct Audio {
    pub tx: Sender<Cmd>,
}

pub static AUDIO: Lazy<Audio> = Lazy::new(|| {
    let (tx, rx) = channel::<Cmd>();

    thread::spawn(move || {
        let Ok((_stream, handle)) = OutputStream::try_default() else {
            eprintln!("Failed to initialize audio output stream.");
            return;
        };

        let mut sinks: Vec<Sink> = Vec::new();

        while let Ok(cmd) = rx.recv() {
            match cmd {
                Cmd::File { path, volume } => {
                    play_file(&handle, &mut sinks, path, volume);
                }
                Cmd::Cleanup => {
                    sinks.retain(|sink| !sink.empty());
                }
            }
        }
    });

    Audio { tx }
});

fn play_file(handle: &OutputStreamHandle, sinks: &mut Vec<Sink>, path: PathBuf, volume: f32) {
    let Ok(sink) = Sink::try_new(handle) else {
        eprintln!("Failed to create audio sink.");
        return;
    };

    let Ok(file) = File::open(&path) else {
        eprintln!("Failed to open audio file: {}", path.display());
        return;
    };

    let Ok(source) = Decoder::new(BufReader::new(file)) else {
        eprintln!("Failed to decode audio file: {}", path.display());
        return;
    };

    sink.set_volume(volume.clamp(0.0, 1.0));
    sink.append(source.fade_in(Duration::from_millis(1)));

    sinks.push(sink);
}

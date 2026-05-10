use std::path::PathBuf;

use crate::app::systems::audio::audio_manager::{AUDIO, Cmd};

pub fn play_audio_from_file(sound: &str, volume: f32) {
    let path = sound_path(sound);

    let _ = AUDIO.tx.send(Cmd::File {
        path,
        volume: volume.clamp(0.0, 1.0),
    });
}

pub fn cleanup() {
    let _ = AUDIO.tx.send(Cmd::Cleanup);
}

fn sound_path(sound: &str) -> PathBuf {
    let relative_path = PathBuf::from("assets")
        .join("sounds")
        .join(format!("{sound}.wav"));

    if cfg!(debug_assertions) {
        return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    }

    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|dir| dir.join(&relative_path)))
        .unwrap_or(relative_path)
}

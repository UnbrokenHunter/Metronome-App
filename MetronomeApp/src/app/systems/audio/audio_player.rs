use crate::app::systems::audio::audio_manager::{Cmd, AUDIO};

pub fn play_audio_from_file(sound: &str, volume: f32) {
    let path = format!("assets/sounds/{}.wav", sound);
    let _ = AUDIO.tx.send(Cmd::File { path, volume });
}

pub fn cleanup() {
    let _ = AUDIO.tx.send(Cmd::Cleanup);
}

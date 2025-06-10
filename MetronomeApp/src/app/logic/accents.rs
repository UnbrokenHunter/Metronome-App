use crate::app::{AppData, types::BeatData};

pub fn calculate_number_of_beats(app: &mut AppData) -> usize {
    return app
        .parameters
        .accents
        .accents
        .iter()
        .map(|accent_data| accent_data.beats.len())
        .sum();
}

pub fn get_beat_at_index(app: &mut AppData, index: usize) -> Option<&mut BeatData> {
    let mut remaining = index;

    for accent in &mut app.parameters.accents.accents {
        if remaining < accent.beats.len() {
            return Some(&mut accent.beats[remaining]);
        } else {
            remaining -= accent.beats.len();
        }
    }

    None // index out of bounds
}

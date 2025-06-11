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

pub fn get_index_of_beat(app: &mut AppData, accent_index: usize, beat_index: usize) -> usize {
    let mut count = 0;

    let mut i = 0;
    while i < accent_index && i < app.parameters.accents.accents.len() {
        count += app.parameters.accents.accents[i].beats.len();
        i += 1;
    }
    count += beat_index;
    return count;
}

pub fn get_accent_and_beat_index(app: &AppData, flat_index: usize) -> Option<(usize, usize)> {
    let mut count = 0;

    for (accent_index, accent) in app.parameters.accents.accents.iter().enumerate() {
        let beat_count = accent.beats.len();

        if flat_index < count + beat_count {
            let beat_index = flat_index - count;
            return Some((accent_index, beat_index));
        }

        count += beat_count;
    }

    None // Out of bounds
}

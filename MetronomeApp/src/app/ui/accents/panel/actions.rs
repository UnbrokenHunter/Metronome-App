use crate::app::{
    AppData,
    types::{AccentData, BeatData, BeatState},
};

pub enum AccentAction {
    MoveUp(usize),
    MoveDown(usize),
    Duplicate(usize),
    Insert(usize),
    Delete(usize),
}

pub fn apply_accent_action(app: &mut AppData, action: AccentAction) {
    let accents = &mut app.parameters.accents.accents;

    match action {
        AccentAction::MoveUp(index) => {
            if index > 0 && index < accents.len() {
                accents.swap(index, index - 1);
            }
        }

        AccentAction::MoveDown(index) => {
            if index + 1 < accents.len() {
                accents.swap(index, index + 1);
            }
        }

        AccentAction::Duplicate(index) => {
            if index < accents.len() {
                accents.insert(index, accents[index].clone());
            }
        }

        AccentAction::Insert(index) => {
            if index <= accents.len() {
                accents.insert(index, default_accent());
            }
        }

        AccentAction::Delete(index) => {
            if index < accents.len() && accents.len() > 1 {
                accents.remove(index);
            }
        }
    }
}

fn default_accent() -> AccentData {
    AccentData {
        beats: vec![BeatData {
            state: BeatState::Downbeat,
        }],
        subdivision: 1,
        name: String::new(),
    }
}
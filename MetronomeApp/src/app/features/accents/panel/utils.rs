use eframe::egui::{self, ImageButton, Ui};

pub const SMALL_ICON_SIZE: f32 = 20.0;
pub const TINY_ICON_SIZE: f32 = 10.0;
pub const SAVE_ICON_SIZE: f32 = 25.0;
pub const BEAT_BUTTON_WIDTH: f32 = 20.0;
pub const BEAT_BUTTON_HEIGHT: f32 = 8.0;

const BEAT_MENU_STATE_MULTIPLIER: usize = 10_000;

pub fn icon_button(ui: &mut Ui, image: egui::ImageSource<'static>, size: f32) -> bool {
    ui.add_sized([size, size], ImageButton::new(image).frame(false))
        .clicked()
}

pub fn truncate(s: &str, max_chars: usize) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if i >= max_chars {
            result.push('…');
            break;
        }

        result.push(c);
    }

    result
}

pub fn beat_menu_state(accent_index: usize, beat_index: usize) -> u32 {
    ((accent_index * BEAT_MENU_STATE_MULTIPLIER) + beat_index + 1) as u32
}
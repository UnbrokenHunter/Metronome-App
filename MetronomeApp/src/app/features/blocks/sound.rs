use super::section::section;
use crate::app::systems::audio;
use crate::app::{AppData, Sounds};
use eframe::egui::{self, Grid, Ui};

const SOUNDS: [Sounds; 6] = [
    Sounds::Beep,
    Sounds::Thump,
    Sounds::Tone,
    Sounds::Clave,
    Sounds::Cowbell,
    Sounds::Drums,
];

pub fn sound(app: &mut AppData, ui: &mut Ui) {
    section(ui, "Sounds:", |ui| {
        ui.horizontal(|ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                sound_grid(app, ui);
            });

            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut app.parameters.volume, 0.0..=1.0).text("Volume"));
            });
        });
    });
}

fn sound_grid(app: &mut AppData, ui: &mut Ui) {
    Grid::new("sound_grid")
        .num_columns(3)
        .spacing([5.0, 10.0])
        .show(ui, |ui| {
            for (index, sound) in SOUNDS.iter().copied().enumerate() {
                if ui
                    .selectable_value(&mut app.parameters.sound, sound, format!("{sound:?}"))
                    .clicked()
                {
                    preview_sound(app, sound);
                }

                if (index + 1).is_multiple_of(3) {
                    ui.end_row();
                }
            }
        });
}

fn preview_sound(app: &mut AppData, sound: Sounds) {
    let sound_name = sound.to_string().to_lowercase();
    audio::play_audio_from_file(&format!("{sound_name}/{sound_name}"), 1f32);
}

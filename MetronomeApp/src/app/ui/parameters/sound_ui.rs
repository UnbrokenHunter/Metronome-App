use crate::app::{AppData, Sounds};
use eframe::egui::{self, Ui};

pub fn sound_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Sounds:");
        ui.separator();

        use crate::app::logic::sound::play_metronome;
        use egui::Grid;
        ui.horizontal(|ui: &mut Ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.vertical(|ui| {
                    Grid::new("sound_grid")
                        .num_columns(3)
                        .spacing([5.0, 10.0])
                        .show(ui, |ui| {
                            for &sound in [
                                Sounds::Beep,
                                Sounds::Thump,
                                Sounds::Tone,
                                Sounds::Clave,
                                Sounds::Cowbell,
                                Sounds::Drums,
                            ]
                            .iter()
                            {
                                if ui
                                    .selectable_value(
                                        &mut app.parameters.sound,
                                        sound,
                                        format!("{:?}", sound),
                                    )
                                    .clicked()
                                {
                                    play_metronome(
                                        app,
                                        format!(
                                            "{}/{}",
                                            sound.to_string().to_lowercase(),
                                            sound.to_string().to_lowercase()
                                        ),
                                    );
                                }

                                // Move to next row every 3 buttons
                                if (sound as usize + 1) % 3 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                });
            });

            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut app.parameters.volume, 0.0..=1.0).text("Volume"));
            });
        });
    });
}

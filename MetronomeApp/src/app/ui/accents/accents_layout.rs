use crate::app::{
    AppData,
    types::{BeatData, BeatState},
};
use eframe::egui::{self, Color32, ImageButton, Ui};

pub fn accents_layout(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading("Accents");
        ui.separator();

        // Box for the Chain
        egui::Frame::group(ui.style()).show(ui, |ui| {
            let mut i = 0;
            while i < app.parameters.accents.accents.len() {
                draw_accent(app, ui, i);
                i += 1;
            }
        });
    });
}

fn draw_accent(app: &mut AppData, ui: &mut Ui, accent_index: usize) {
    let accent = app
        .parameters
        .accents
        .accents
        .get_mut(accent_index)
        .unwrap();

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading(format!("{} Beats", accent.beats.len()));
        ui.separator();
        ui.horizontal(|ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                for (i, beat) in accent.beats.iter_mut().enumerate() {
                    let custom_color = match beat.state {
                        BeatState::Strong => Color32::from_rgb(100, 40, 40), // dark red
                        BeatState::Weak => Color32::from_rgb(40, 60, 100),   // dark blue
                        BeatState::Off => Color32::from_rgb(30, 30, 30),     // dark gray
                    };

                    let response = ui.add(
                        egui::Button::new("")
                            .fill(custom_color)
                            .min_size(egui::vec2(20.0, 20.0)),
                    );

                    if response.clicked() {
                        // Left click: cycle forward
                        beat.state = match beat.state {
                            BeatState::Off => BeatState::Strong,
                            BeatState::Strong => BeatState::Weak,
                            BeatState::Weak => BeatState::Off,
                        };
                        println!("Left click: Beat {} → {:?}", i, beat.state);
                    }

                    if response.secondary_clicked() {
                        // Right click: cycle backward
                        beat.state = match beat.state {
                            BeatState::Off => BeatState::Weak,
                            BeatState::Weak => BeatState::Strong,
                            BeatState::Strong => BeatState::Off,
                        };
                        println!("Right click: Beat {} → {:?}", i, beat.state);
                    }
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                egui::Frame::group(ui.style())
                    .corner_radius(5)
                    .fill(Color32::from_rgb(40, 40, 40))
                    .show(ui, |ui| {
                        let size = [15.0, 15.0];
                        ui.horizontal(|ui: &mut Ui| {
                            if ui
                                .add_sized(
                                    size,
                                    ImageButton::new(egui::include_image!(
                                        "../../../../assets/images/minus.png"
                                    ))
                                    .frame(false),
                                )
                                .clicked()
                            {
                                println!("Minus clicked!");
                                accent.beats.pop();
                            }

                            if ui
                                .add_sized(
                                    size,
                                    ImageButton::new(egui::include_image!(
                                        "../../../../assets/images/plus.png"
                                    ))
                                    .frame(false),
                                )
                                .clicked()
                            {
                                println!("Plus clicked!");
                                accent.beats.push(BeatData {
                                    state: BeatState::Strong,
                                });
                            }
                        });
                    });
            });
        });
    });
}

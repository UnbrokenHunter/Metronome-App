use crate::app::{
    AppData,
    types::{AccentData, BeatData, BeatState},
};
use eframe::egui::{self, Color32, ImageButton, Margin, ScrollArea, Ui};

pub fn accents_layout(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading("Accents");
        ui.separator();

        // Box for the Chain
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui: &mut Ui| {
                    insert_accent_button(app, ui, 0);

                    let mut i = 0;
                    while i < app.parameters.accents.accents.len() {
                        draw_accent(app, ui, i);
                        insert_accent_button(app, ui, i + 1);
                        i += 1;
                    }
                });
            });
        });
    });
}

fn insert_accent_button(app: &mut AppData, ui: &mut Ui, i: usize) {
    egui::Frame::group(ui.style())
        .corner_radius(5)
        .fill(egui::Color32::from_rgb(40, 40, 40))
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 2,
            bottom: 2,
        })
        .show(ui, |ui| {
            if ui
                .add_sized(
                    [20.0, 20.0],
                    ImageButton::new(egui::include_image!(
                        "../../../../assets/images/down_arrow.png"
                    ))
                    .frame(false),
                )
                .clicked()
            {
                println!("Insert clicked!");
                app.parameters
                    .accents
                    .accents
                    .insert(i, AccentData { beats: vec![] });
            }
        });
}

fn draw_accent(app: &mut AppData, ui: &mut Ui, accent_index: usize) {
    let accent = app
        .parameters
        .accents
        .accents
        .get_mut(accent_index)
        .unwrap();

    let mut to_delete: Option<usize> = None;

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui: &mut Ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.heading(format!("{} Beats", accent.beats.len()));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui
                    .add_sized(
                        [20.0, 20.0],
                        ImageButton::new(egui::include_image!(
                            "../../../../assets/images/trash.png"
                        ))
                        .frame(false),
                    )
                    .clicked()
                {
                    println!("Trash clicked!");
                    to_delete = Some(accent_index as usize);
                }
            });
        });
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
                        egui::Button::new(format!("{}", i + 1))
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
                        let size = [10.0, 10.0];
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
    if let Some(index) = to_delete {
        if index < app.parameters.accents.accents.len() {
            app.parameters.accents.accents.remove(index);
        }
    }
}

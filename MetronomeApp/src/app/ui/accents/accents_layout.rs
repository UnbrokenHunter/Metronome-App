use crate::app::{
    AppData,
    logic::accents::get_accent_and_beat_index,
    types::{AccentData, BeatData, BeatState},
};
use eframe::egui::{
    self, Align, Color32, ComboBox, ImageButton, Layout, ScrollArea, TextEdit, TextStyle, Ui,
};

pub fn accents_layout(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            // Save Button
            if ui
                .add_sized(
                    [25.0, 25.0],
                    ImageButton::new(egui::include_image!("../../../../assets/images/save.png"))
                        .frame(false),
                )
                .clicked()
            {
                app.accent_presets
                    .accent_chains
                    .push(app.parameters.accents.clone());
            }

            // Allocate space for middle and right
            let available_width = ui.available_width();

            // Begin a horizontal layout that fills the remaining space
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                // Header - take up all remaining space except for the dropdown (e.g., 150px)
                let dropdown_width = 180.0;
                let header_width = available_width - dropdown_width;

                ui.add_sized(
                    [header_width.max(0.0), 27.0],
                    TextEdit::singleline(&mut app.parameters.accents.name)
                        .font(TextStyle::Heading)
                        .hint_text("Title..."),
                );

                // Presets Dropdown
                ComboBox::from_label("")
                    .selected_text(truncate(&app.parameters.accents.name, 22))
                    .height(300.0)
                    .show_ui(ui, |ui| {
                        for preset in &app.accent_presets.accent_chains {
                            if ui
                                .selectable_label(
                                    app.parameters.accents.name == preset.name,
                                    &preset.name,
                                )
                                .clicked()
                            {
                                app.parameters.accents = preset.clone();
                            }
                        }
                    });

                fn truncate(s: &str, max_chars: usize) -> String {
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
            });
        });
        ui.separator();

        // Box for the Chain
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui: &mut Ui| {
                    let mut i = 0;
                    let total_width = ui.available_width();

                    while i < app.parameters.accents.accents.len() {
                        draw_accent(app, ui, i, total_width);
                        i += 1;
                    }
                });
            });
        });
    });
}

fn draw_accent(app: &mut AppData, ui: &mut Ui, accent_index: usize, total_width: f32) {
    let current_click = get_accent_and_beat_index(app, app.runtime.last_click_accent as usize);

    let accent = app
        .parameters
        .accents
        .accents
        .get_mut(accent_index)
        .unwrap();

    let mut to_delete: Option<usize> = None;
    let mut to_duplicate: Option<usize> = None;
    let mut to_insert: Option<usize> = None;
    let mut to_move_up: Option<usize> = None;
    let mut to_move_down: Option<usize> = None;

    ui.horizontal(|ui: &mut Ui| {
        egui::Grid::new(format!("{}_insert_grid", accent_index))
            .num_columns(1) // optional, inferred by usage
            .striped(false)
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
                    println!("First Insert clicked!");
                    to_insert = Some(accent_index);
                }
                ui.end_row(); // <-- marks end of the row
                if ui
                    .add_sized(
                        [20.0, 20.0],
                        ImageButton::new(egui::include_image!(
                            "../../../../assets/images/duplicate.png"
                        ))
                        .frame(false),
                    )
                    .clicked()
                {
                    println!("Duplicate Clicked!");
                    to_duplicate = Some(accent_index);
                }
                ui.end_row(); // <-- marks end of the row
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
                    println!("Second Insert clicked!");
                    to_insert = Some(accent_index + 1);
                }
                ui.end_row(); // <-- marks end of the row
            });

        ui.vertical(|ui: &mut Ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            [20.0, 20.0],
                            ImageButton::new(egui::include_image!(
                                "../../../../assets/images/up.png"
                            ))
                            .frame(false),
                        )
                        .clicked()
                    {
                        println!("Move Accent Up Clicked!");
                        to_move_up = Some(accent_index);
                    }

                    // === Left-aligned: Beat count ===
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        ui.add_sized(
                            [total_width / 2.0, 20.0],
                            TextEdit::singleline(&mut accent.name)
                                .font(TextStyle::Heading)
                                .hint_text(format!("{} Beats", accent.beats.len())),
                        );
                    });

                    // === Spacer: push subdivision to ~60% across
                    let target_offset = total_width * 0.60;
                    let used_width = ui.min_size().x;
                    let spacer_width = (target_offset - used_width).max(0.0);
                    ui.add_space(spacer_width);

                    // === Center-right: Subdivision ===
                    ui.horizontal(|ui| {
                        ui.label("Subdivision:");
                        for value in [1, 2, 3, 4, 6, 8] {
                            ui.selectable_value(&mut accent.subdivision, value, value.to_string());
                        }
                    });

                    // === Right-aligned: Trash icon ===
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
                            to_delete = Some(accent_index);
                        }
                    });
                });
                ui.separator();
                ui.horizontal(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        if ui
                            .add_sized(
                                [20.0, 20.0],
                                ImageButton::new(egui::include_image!(
                                    "../../../../assets/images/down.png"
                                ))
                                .frame(false),
                            )
                            .clicked()
                        {
                            println!("Move Accent Down Clicked!");
                            to_move_down = Some(accent_index);
                        }

                        for (i, beat) in accent.beats.iter_mut().enumerate() {
                            let custom_color = if current_click == Some((accent_index, i))
                                && app.runtime.playing
                            {
                                Color32::from_rgb(60, 70, 100)
                            } else {
                                match beat.state {
                                    BeatState::Downbeat => Color32::from_rgb(80, 80, 120), // Desaturated indigo
                                    BeatState::Strong => Color32::from_rgb(130, 130, 130), // Light gray
                                    BeatState::Weak => Color32::from_rgb(80, 80, 80), // Mid gray
                                    BeatState::Off => Color32::from_rgb(40, 40, 40),  // Dark gray
                                }
                            };

                            let response = ui.add(
                                egui::Button::new(format!("{}", i + 1))
                                    .fill(custom_color)
                                    .min_size(egui::vec2(20.0, 20.0)),
                            );

                            if response.clicked() {
                                // Left click: cycle forward
                                beat.state = match beat.state {
                                    BeatState::Off => BeatState::Downbeat,
                                    BeatState::Downbeat => BeatState::Strong,
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
                                    BeatState::Strong => BeatState::Downbeat,
                                    BeatState::Downbeat => BeatState::Off,
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
                                        && accent.beats.len() > 1
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
        });
    });
    if let Some(index) = to_move_up {
        if index > 0 && index < app.parameters.accents.accents.len() {
            app.parameters.accents.accents.swap(index, index - 1);
        }
    }
    if let Some(index) = to_move_down {
        if index + 1 < app.parameters.accents.accents.len() {
            app.parameters.accents.accents.swap(index, index + 1);
        }
    }

    if let Some(index) = to_duplicate {
        if index < app.parameters.accents.accents.len() {
            app.parameters
                .accents
                .accents
                .insert(index, app.parameters.accents.accents[index].clone());
        }
    }

    if let Some(index) = to_insert {
        println!("Index: {}", index);
        if index < app.parameters.accents.accents.len() {
            app.parameters.accents.accents.insert(
                // TODO FIX SECOND INSERT NOT WORKING
                accent_index,
                AccentData {
                    beats: vec![BeatData {
                        state: BeatState::Downbeat,
                    }],
                    subdivision: 1,
                    name: "".to_owned(),
                },
            );
        } else if index == app.parameters.accents.accents.len() {
            app.parameters.accents.accents.push(AccentData {
                beats: vec![BeatData {
                    state: BeatState::Downbeat,
                }],
                subdivision: 1,
                name: "".to_owned(),
            });
        }
    }

    if let Some(index) = to_delete {
        if index < app.parameters.accents.accents.len() {
            app.parameters.accents.accents.remove(index);
        }
    }
}

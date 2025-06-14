/*
 * To anyone whoo ever has to edit this file im so sorry ;-;
*/

use crate::app::{
    AppData,
    logic::accents::get_accent_and_beat_index,
    types::{AccentData, BeatData, BeatState},
};
use eframe::egui::{
    self, Align, Color32, ComboBox, CornerRadius, Frame, ImageButton, Layout, Rect, ScrollArea,
    Sense, Stroke, TextEdit, TextStyle, Ui, Vec2,
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
                            [total_width / 2.5, 20.0],
                            TextEdit::singleline(&mut accent.name)
                                .font(TextStyle::Heading)
                                .hint_text(format!("{} Beats", accent.beats.len())),
                        );
                    });

                    // === Spacer: push subdivision to ~40% across
                    let target_offset = total_width * 0.40;
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

                        let old_spacing = ui.spacing().item_spacing;
                        ui.spacing_mut().item_spacing.y = 0.0;

                        // This is where the selectoin of beat type is
                        for (i, beat) in accent.beats.iter_mut().enumerate() {
                            let button_size: Vec2 = Vec2::new(20.0, 8.0);
                            let start_pos = ui.cursor().min;
                            let menu_state = ((accent_index * 10_000) + i + 1) as u32; // ID Unique to this beat of this accent, can be used to identify it. 10,000 is just a big number to differentiate accent index from i

                            let mut cursor = start_pos;

                            let radius: u8 = 2;

                            let rounding_top = CornerRadius {
                                nw: radius,
                                ne: radius,
                                sw: 0,
                                se: 0,
                            };
                            let rounding_mid = CornerRadius::ZERO;
                            let rounding_bot = CornerRadius {
                                nw: 0,
                                ne: 0,
                                sw: radius,
                                se: radius,
                            };

                            let override_color =
                                Color32::from_hex(&app.settings.color_scheme.override_color)
                                    .unwrap();
                            let downbeat_color =
                                Color32::from_hex(&app.settings.color_scheme.downbeat_color)
                                    .unwrap();
                            let strong_color =
                                Color32::from_hex(&app.settings.color_scheme.strong_color).unwrap();
                            let weak_color =
                                Color32::from_hex(&app.settings.color_scheme.weak_color).unwrap();
                            let off_color =
                                Color32::from_hex(&app.settings.color_scheme.off_color).unwrap();

                            let states = [
                                (BeatState::Downbeat, downbeat_color, rounding_top),
                                (BeatState::Strong, strong_color, rounding_mid),
                                (BeatState::Weak, weak_color, rounding_mid),
                                (BeatState::Off, off_color, rounding_bot),
                            ];

                            let invisible_frame = Frame {
                                // TODO FIX ONLY SOME COLORS BEING SHOWN IF HOVER ON THE EDGE
                                fill: Color32::TRANSPARENT, // no background
                                stroke: egui::Stroke::NONE, // no border
                                corner_radius: CornerRadius::ZERO, // no rounding
                                outer_margin: egui::Margin::default(),
                                inner_margin: egui::Margin::default(),
                                ..Default::default()
                            };

                            let response = invisible_frame.show(ui, |ui| {
                                for (_j, (state, color, rounding)) in states.iter().enumerate() {
                                    let rect = Rect::from_min_size(cursor, button_size);
                                    let selected_color = match beat.state {
                                        BeatState::Downbeat => downbeat_color,
                                        BeatState::Strong => strong_color,
                                        BeatState::Weak => weak_color,
                                        BeatState::Off => off_color,
                                    };

                                    let response = ui.allocate_rect(rect, Sense::click());
                                    if response.hovered() {
                                        app.runtime.menu_state = menu_state;
                                    }

                                    let fill = if current_click == Some((accent_index, i))
                                        && app.runtime.playing
                                    {
                                        override_color
                                    } else if !(app.runtime.menu_state == menu_state) {
                                        selected_color
                                    } else {
                                        *color
                                    };

                                    ui.painter().rect(
                                        rect,
                                        *rounding,
                                        fill,
                                        Stroke::NONE,
                                        egui::StrokeKind::Middle,
                                    );

                                    if response.clicked() {
                                        beat.state = *state;
                                    }

                                    cursor.y += button_size.y;
                                }

                                let painter = ui.painter();

                                // Full bounding rect of the 4 stacked buttons
                                let total_height = button_size.y * states.len() as f32;
                                let full_rect = Rect::from_min_size(
                                    start_pos,
                                    Vec2::new(button_size.x, total_height),
                                );

                                // Text to draw
                                let label = format!("{}", i + 1);

                                painter.text(
                                    full_rect.center(),                          // position: center of all 4 rects
                                    egui::Align2::CENTER_CENTER, // alignment: center
                                    label,                       // text
                                    egui::TextStyle::Button.resolve(ui.style()), // style
                                    Color32::WHITE,              // text color
                                );
                            });

                            if !(response.response.hovered()) // TODO Make the ui not freak out when you click because it looses hovering for a second
                                && app.runtime.menu_state == menu_state
                            {
                                app.runtime.menu_state = 0;
                            }
                        }

                        ui.spacing_mut().item_spacing = old_spacing;
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        egui::Frame::group(ui.style())
                            .corner_radius(5)
                            .fill(Color32::from_hex(&app.settings.color_scheme.weak_color).unwrap())
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
        if index < app.parameters.accents.accents.len() {
            app.parameters.accents.accents.insert(
                index,
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
        if index < app.parameters.accents.accents.len() && app.parameters.accents.accents.len() > 1
        {
            app.parameters.accents.accents.remove(index);
        }
    }
}

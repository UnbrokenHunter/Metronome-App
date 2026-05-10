use super::{
    actions::{apply_accent_action, AccentAction},
    beat_column::draw_beat_column,
    utils::{icon_button, SMALL_ICON_SIZE, TINY_ICON_SIZE},
};
use crate::app::data::BeatColors;
use crate::app::{
    logic::accents::get_accent_and_beat_index, AccentData, AppData, BeatData, BeatState,
};
use eframe::egui::{self, Align, Frame, Layout, TextEdit, TextStyle, Ui};

pub fn draw_accent(app: &mut AppData, ui: &mut Ui, accent_index: usize, total_width: f32) {
    let current_click = get_accent_and_beat_index(app, app.runtime.last_click_accent as usize);
    let colors = app.current_theme().beat;
    let playing = app.runtime.playing;

    let mut action = None;

    let menu_state = &mut app.runtime.menu_state;
    let accent = app
        .parameters
        .accents
        .accents
        .get_mut(accent_index)
        .unwrap();

    ui.horizontal(|ui| {
        draw_accent_side_buttons(ui, accent_index, &mut action);

        ui.vertical(|ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                draw_accent_header(ui, accent, accent_index, total_width, &mut action);

                ui.separator();

                draw_accent_beats(
                    ui,
                    accent,
                    accent_index,
                    menu_state,
                    playing,
                    current_click,
                    &colors,
                    &mut action,
                );
            });
        });
    });

    if let Some(action) = action {
        apply_accent_action(app, action);
    }
}

fn draw_accent_side_buttons(ui: &mut Ui, accent_index: usize, action: &mut Option<AccentAction>) {
    egui::Grid::new(format!("{accent_index}_insert_grid"))
        .num_columns(1)
        .striped(false)
        .show(ui, |ui| {
            if icon_button(
                ui,
                egui::include_image!("../../../../../../assets/icons/down_arrow.png"),
                SMALL_ICON_SIZE,
            ) {
                *action = Some(AccentAction::Insert(accent_index));
            }

            ui.end_row();

            if icon_button(
                ui,
                egui::include_image!("../../../../../../assets/icons/duplicate.png"),
                SMALL_ICON_SIZE,
            ) {
                *action = Some(AccentAction::Duplicate(accent_index));
            }

            ui.end_row();

            if icon_button(
                ui,
                egui::include_image!("../../../../../../assets/icons/down_arrow.png"),
                SMALL_ICON_SIZE,
            ) {
                *action = Some(AccentAction::Insert(accent_index + 1));
            }

            ui.end_row();
        });
}

fn draw_accent_header(
    ui: &mut Ui,
    accent: &mut AccentData,
    accent_index: usize,
    total_width: f32,
    action: &mut Option<AccentAction>,
) {
    ui.horizontal(|ui| {
        if icon_button(
            ui,
            egui::include_image!("../../../../../../assets/icons/up.png"),
            SMALL_ICON_SIZE,
        ) {
            *action = Some(AccentAction::MoveUp(accent_index));
        }

        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.add_sized(
                [total_width / 2.5, 20.0],
                TextEdit::singleline(&mut accent.name)
                    .font(TextStyle::Heading)
                    .hint_text(format!("{} Beats", accent.beats.len())),
            );
        });

        let target_offset = total_width * 0.40;
        let used_width = ui.min_size().x;
        let spacer_width = (target_offset - used_width).max(0.0);
        ui.add_space(spacer_width);

        ui.horizontal(|ui| {
            ui.label("Subdivision:");

            for value in [1, 2, 3, 4, 6, 8] {
                ui.selectable_value(&mut accent.subdivision, value, value.to_string());
            }
        });

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            if icon_button(
                ui,
                egui::include_image!("../../../../../../assets/icons/trash.png"),
                SMALL_ICON_SIZE,
            ) {
                *action = Some(AccentAction::Delete(accent_index));
            }
        });
    });
}

#[allow(clippy::too_many_arguments)]
fn draw_accent_beats(
    ui: &mut Ui,
    accent: &mut AccentData,
    accent_index: usize,
    menu_state: &mut u32,
    playing: bool,
    current_click: Option<(usize, usize)>,
    colors: &BeatColors,
    action: &mut Option<AccentAction>,
) {
    ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            if icon_button(
                ui,
                egui::include_image!("../../../../../../assets/icons/down.png"),
                SMALL_ICON_SIZE,
            ) {
                *action = Some(AccentAction::MoveDown(accent_index));
            }

            let old_spacing = ui.spacing().item_spacing;
            ui.spacing_mut().item_spacing.y = 0.0;

            for (beat_index, beat) in accent.beats.iter_mut().enumerate() {
                draw_beat_column(
                    ui,
                    beat,
                    accent_index,
                    beat_index,
                    menu_state,
                    playing,
                    current_click,
                    colors,
                );
            }

            ui.spacing_mut().item_spacing = old_spacing;
        });

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            draw_beat_count_controls(ui, accent, colors);
        });
    });
}

fn draw_beat_count_controls(ui: &mut Ui, accent: &mut AccentData, colors: &BeatColors) {
    Frame::group(ui.style())
        .corner_radius(5)
        .fill(colors.weak_color)
        .show(ui, |ui| {
            let size = [TINY_ICON_SIZE, TINY_ICON_SIZE];

            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        size,
                        egui::ImageButton::new(egui::include_image!(
                            "../../../../../../assets/icons/minus.png"
                        ))
                        .frame(false),
                    )
                    .clicked()
                    && accent.beats.len() > 1
                {
                    accent.beats.pop();
                }

                if ui
                    .add_sized(
                        size,
                        egui::ImageButton::new(egui::include_image!(
                            "../../../../../../assets/icons/plus.png"
                        ))
                        .frame(false),
                    )
                    .clicked()
                {
                    accent.beats.push(BeatData {
                        state: BeatState::Strong,
                    });
                }
            });
        });
}

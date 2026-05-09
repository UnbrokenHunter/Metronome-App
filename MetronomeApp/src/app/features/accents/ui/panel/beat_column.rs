use crate::app::BeatData;
use crate::app::systems::colors::theme_presets::Theme;
use eframe::egui::{
    self, Color32, CornerRadius, Frame, Rect, Sense, Stroke, TextStyle, Ui, Vec2,
};

use super::{
    colors::{beat_state_rows, selected_beat_color},
    utils::{beat_menu_state, BEAT_BUTTON_HEIGHT, BEAT_BUTTON_WIDTH},
};

#[allow(clippy::too_many_arguments)]
pub fn draw_beat_column(
    ui: &mut Ui,
    beat: &mut BeatData,
    accent_index: usize,
    beat_index: usize,
    menu_state: &mut u32,
    playing: bool,
    current_click: Option<(usize, usize)>,
    colors: &Theme,
) {
    let button_size = Vec2::new(BEAT_BUTTON_WIDTH, BEAT_BUTTON_HEIGHT);
    let start_pos = ui.cursor().min;
    let menu_state_id = beat_menu_state(accent_index, beat_index);

    let mut cursor = start_pos;

    let states = beat_state_rows(colors);
    let state_count = states.len();

    let invisible_frame = Frame {
        fill: Color32::TRANSPARENT,
        stroke: Stroke::NONE,
        corner_radius: CornerRadius::ZERO,
        outer_margin: egui::Margin::default(),
        inner_margin: egui::Margin::default(),
        ..Default::default()
    };

    let response = invisible_frame.show(ui, |ui| {
        for (state, color, rounding) in states {
            let rect = Rect::from_min_size(cursor, button_size);
            let response = ui.allocate_rect(rect, Sense::click());

            if response.hovered() {
                *menu_state = menu_state_id;
            }

            let selected_color = selected_beat_color(beat.state, colors);

            let fill = if current_click == Some((accent_index, beat_index)) && playing {
                colors.override_color
            } else if *menu_state != menu_state_id {
                selected_color
            } else {
                color
            };

            ui.painter()
                .rect(rect, rounding, fill, Stroke::NONE, egui::StrokeKind::Middle);

            if response.clicked() {
                beat.state = state;
            }

            cursor.y += button_size.y;
        }

        draw_beat_number(ui, start_pos, button_size, state_count, beat_index);
    });

    if !response.response.hovered() && *menu_state == menu_state_id {
        *menu_state = 0;
    }
}

fn draw_beat_number(
    ui: &mut Ui,
    start_pos: egui::Pos2,
    button_size: Vec2,
    state_count: usize,
    beat_index: usize,
) {
    let total_height = button_size.y * state_count as f32;
    let full_rect = Rect::from_min_size(start_pos, Vec2::new(button_size.x, total_height));

    ui.painter().text(
        full_rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{}", beat_index + 1),
        TextStyle::Button.resolve(ui.style()),
        Color32::WHITE,
    );
}
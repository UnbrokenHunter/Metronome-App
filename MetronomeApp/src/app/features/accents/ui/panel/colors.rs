use crate::app::BeatState;
use crate::app::data::BeatColors;
use eframe::egui::{Color32, CornerRadius};

pub(super) fn beat_state_rows(colors: &BeatColors) -> [(BeatState, Color32, CornerRadius); 4] {
    let radius = 2;

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

    [
        (BeatState::Downbeat, colors.downbeat_color, rounding_top),
        (BeatState::Strong, colors.strong_color, rounding_mid),
        (BeatState::Weak, colors.weak_color, rounding_mid),
        (BeatState::Off, colors.off_color, rounding_bot),
    ]
}

pub(super) fn selected_beat_color(state: BeatState, colors: &BeatColors) -> Color32 {
    match state {
        BeatState::Downbeat => colors.downbeat_color,
        BeatState::Strong => colors.strong_color,
        BeatState::Weak => colors.weak_color,
        BeatState::Off => colors.off_color,
    }
}

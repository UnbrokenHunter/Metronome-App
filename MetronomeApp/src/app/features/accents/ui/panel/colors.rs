use crate::app::{AppData, types::BeatState};
use eframe::egui::{Color32, CornerRadius};

#[derive(Clone, Copy)]
pub(super) struct BeatColors {
    pub override_color: Color32,
    pub downbeat: Color32,
    pub strong: Color32,
    pub weak: Color32,
    pub off: Color32,
}

pub(super) fn beat_colors(app: &AppData) -> BeatColors {
    let scheme = &app.settings.color_scheme;

    BeatColors {
        override_color: Color32::from_hex(&scheme.override_color).unwrap(),
        downbeat: Color32::from_hex(&scheme.downbeat_color).unwrap(),
        strong: Color32::from_hex(&scheme.strong_color).unwrap(),
        weak: Color32::from_hex(&scheme.weak_color).unwrap(),
        off: Color32::from_hex(&scheme.off_color).unwrap(),
    }
}

pub(super) fn beat_state_rows(colors: BeatColors) -> [(BeatState, Color32, CornerRadius); 4] {
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
        (BeatState::Downbeat, colors.downbeat, rounding_top),
        (BeatState::Strong, colors.strong, rounding_mid),
        (BeatState::Weak, colors.weak, rounding_mid),
        (BeatState::Off, colors.off, rounding_bot),
    ]
}

pub(super) fn selected_beat_color(state: BeatState, colors: BeatColors) -> Color32 {
    match state {
        BeatState::Downbeat => colors.downbeat,
        BeatState::Strong => colors.strong,
        BeatState::Weak => colors.weak,
        BeatState::Off => colors.off,
    }
}

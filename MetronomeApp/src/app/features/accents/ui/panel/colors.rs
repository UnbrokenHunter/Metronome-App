use crate::app::{BeatState};
use eframe::egui::{Color32, CornerRadius};
use crate::app::systems::colors::theme_presets::Theme;

pub(super) fn beat_state_rows(theme: &Theme) -> [(BeatState, Color32, CornerRadius); 4] {
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
        (BeatState::Downbeat, theme.downbeat_color, rounding_top),
        (BeatState::Strong, theme.strong_color, rounding_mid),
        (BeatState::Weak, theme.weak_color, rounding_mid),
        (BeatState::Off, theme.off_color, rounding_bot),
    ]
}

pub(super) fn selected_beat_color(state: BeatState, theme: &Theme) -> Color32 {
    match state {
        BeatState::Downbeat => theme.downbeat_color,
        BeatState::Strong => theme.strong_color,
        BeatState::Weak => theme.weak_color,
        BeatState::Off => theme.off_color,
    }
}

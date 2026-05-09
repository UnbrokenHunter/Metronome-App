use crate::app::data::AppThemeData;
use crate::app::systems::colors::theme_presets::Theme;

fn clamp_index(themes: &AppThemeData, idx: usize) -> usize {
    let n = themes.len();

    if n == 0 {
        0
    } else {
        idx.min(n - 1)
    }
}

pub fn theme(themes: &AppThemeData, selected_index: usize) -> Theme {
    let idx = clamp_index(themes, selected_index);

    themes
        .get(idx)
        .expect("at least one theme present")
        .clone()
}

pub fn all_themes(themes: &AppThemeData) -> &[Theme] {
    themes.all()
}
use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppThemeData(pub Vec<Theme>);

impl AppThemeData {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_checked(&self, index: usize) -> Option<&Theme> {
        if self.0.is_empty() {
            return None;
        }

        self.0.get(index.min(self.0.len() - 1))
    }

    pub fn get(&self, index: usize) -> &Theme {
        self.get_checked(index).expect("at least one theme present")
    }

    pub fn all(&self) -> &[Theme] {
        &self.0
    }

    fn clamp_index(&self, idx: usize) -> usize {
        let n = self.len();

        if n == 0 { 0 } else { idx.min(n - 1) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub white: Color32,
    pub black: Color32,

    pub beat: BeatColors,

    pub ui: Option<UITheme>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct BeatColors {
    pub override_color: Color32,
    pub downbeat_color: Color32,
    pub strong_color: Color32,
    pub weak_color: Color32,
    pub off_color: Color32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct UITheme {
    pub extreme_bg_color: Color32,
    pub panel_fill: Color32,
    pub window_fill: Color32,
    pub faint_bg_color: Color32,
    pub override_text_color: Color32,
    pub hyperlink_color: Color32,
    pub selection_bg: Color32,
    pub selection_stroke: Color32,
    pub hovered_bg: Color32,
    pub active_bg: Color32,
    pub open_bg: Color32,
}

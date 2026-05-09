use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub white: Color32,
    pub black: Color32,
    pub override_color: Color32,
    pub downbeat_color: Color32,
    pub strong_color: Color32,
    pub weak_color: Color32,
    pub off_color: Color32,

    pub ui: Option<UITheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
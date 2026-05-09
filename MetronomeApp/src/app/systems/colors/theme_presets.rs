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
    pub graph: GraphTheme,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphTheme {
    pub background: Color32,
    pub node_gradient: Vec<Color32>,
    pub edge_gradient: Vec<Color32>,
    pub cadence_gradient: Vec<Color32>,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "Light".into(),
            white: Color32::from_hex("#C8D4F0").unwrap(),
            black: Color32::from_hex("#A0A0E0").unwrap(),
            override_color: Color32::from_hex("#C8D4F0").unwrap(),
            downbeat_color: Color32::from_hex("#A0A0E0").unwrap(),
            strong_color: Color32::from_hex("#555555").unwrap(),
            weak_color: Color32::from_hex("#888888").unwrap(),
            off_color: Color32::from_hex("#CCCCCC").unwrap(),
            ui: None,
            graph: GraphTheme {
                background: Color32::from_hex("#F9F9F9").unwrap(),
                node_gradient: vec![
                    Color32::from_hex("#5555AA").unwrap(),
                    Color32::from_hex("#77CC88").unwrap(),
                    Color32::from_hex("#AAAAFF").unwrap(),
                ],
                edge_gradient: vec![
                    Color32::from_hex("#CCCCCC").unwrap(),
                    Color32::from_hex("#888888").unwrap(),
                ],
                cadence_gradient: vec![
                    Color32::from_hex("#FF0000").unwrap(),
                    Color32::from_hex("#000000").unwrap(),
                ],
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "Dark".into(),
            white: Color32::from_hex("#a6b2d6ff").unwrap(),
            black: Color32::from_hex("#303053ff").unwrap(),
            override_color: Color32::from_hex("#3C4664").unwrap(),
            downbeat_color: Color32::from_hex("#505078").unwrap(),
            strong_color: Color32::from_hex("#828282").unwrap(),
            weak_color: Color32::from_hex("#505050").unwrap(),
            off_color: Color32::from_hex("#282828").unwrap(),
            ui: None,
            graph: GraphTheme {
                background: Color32::from_hex("#050A1E").unwrap(),
                node_gradient: vec![
                    Color32::from_hex("#002814").unwrap(),
                    Color32::from_hex("#00A064").unwrap(),
                    Color32::from_hex("#50FF78").unwrap(),
                ],
                edge_gradient: vec![
                    Color32::from_hex("#6f58b4ff").unwrap(),
                    Color32::from_hex("#ffc400ff").unwrap(),
                ],
                cadence_gradient: vec![
                    Color32::from_hex("#6f58b4ff").unwrap(),
                    Color32::from_hex("#ff0059ff").unwrap(),
                ],
            },
        }
    }

    pub fn pastel() -> Self {
        Self {
            name: "Pastel".into(),
            white: Color32::from_hex("#3C4664").unwrap(),
            black: Color32::from_hex("#cce2a6").unwrap(),
            override_color: Color32::from_hex("#3C4664").unwrap(),
            downbeat_color: Color32::from_hex("#cce2a6").unwrap(),
            strong_color: Color32::from_hex("#a3d3e0").unwrap(),
            weak_color: Color32::from_hex("#a0abde").unwrap(),
            off_color: Color32::from_hex("#8b86c6").unwrap(),
            ui: Some(UITheme {
                extreme_bg_color: Color32::from_hex("#FFF7EA").unwrap(),
                panel_fill: Color32::from_hex("#FAE6D7").unwrap(),
                window_fill: Color32::from_hex("#F2DCCC").unwrap(),
                faint_bg_color: Color32::from_hex("#FFF7EA").unwrap(),
                override_text_color: Color32::from_hex("#503C32").unwrap(),
                hyperlink_color: Color32::from_hex("#FFB4AA").unwrap(),
                selection_bg: Color32::from_hex("#B4D2E6").unwrap(),
                selection_stroke: Color32::from_hex("#b69255ff").unwrap(),
                hovered_bg: Color32::from_hex("#FFB4AA").unwrap(),
                active_bg: Color32::from_hex("#B4D2E6").unwrap(),
                open_bg: Color32::from_hex("#DCB4BE").unwrap(),
            }),
            graph: GraphTheme {
                background: Color32::from_hex("#FFF7EA").unwrap(),
                node_gradient: vec![
                    Color32::from_hex("#FFB4AA").unwrap(),
                    Color32::from_hex("#DCB4BE").unwrap(),
                    Color32::from_hex("#B4D2E6").unwrap(),
                ],
                edge_gradient: vec![
                    Color32::from_hex("#A0ABDE").unwrap(),
                    Color32::from_hex("#8B86C6").unwrap(),
                ],
                cadence_gradient: vec![
                    Color32::from_hex("#D81159").unwrap(),
                    Color32::from_hex("#8F2D56").unwrap(),
                ],
            },
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".into(),
            white: Color32::from_hex("#D8DEE9").unwrap(),
            black: Color32::from_hex("#88C0D0").unwrap(),
            override_color: Color32::from_hex("#D8DEE9").unwrap(),
            downbeat_color: Color32::from_hex("#88C0D0").unwrap(),
            strong_color: Color32::from_hex("#81A1C1").unwrap(),
            weak_color: Color32::from_hex("#5E81AC").unwrap(),
            off_color: Color32::from_hex("#2E3440").unwrap(),
            ui: Some(UITheme {
                extreme_bg_color: Color32::from_hex("#2E3440").unwrap(),
                panel_fill: Color32::from_hex("#3B4252").unwrap(),
                window_fill: Color32::from_hex("#434C5E").unwrap(),
                faint_bg_color: Color32::from_hex("#4C566A").unwrap(),
                override_text_color: Color32::from_hex("#D8DEE9").unwrap(),
                hyperlink_color: Color32::from_hex("#EBCB8B").unwrap(),
                selection_bg: Color32::from_hex("#BF616A").unwrap(),
                selection_stroke: Color32::from_hex("#4C566A").unwrap(),
                hovered_bg: Color32::from_hex("#A3BE8C").unwrap(),
                active_bg: Color32::from_hex("#88C0D0").unwrap(),
                open_bg: Color32::from_hex("#B48EAD").unwrap(),
            }),
            graph: GraphTheme {
                background: Color32::from_hex("#2E3440").unwrap(),
                node_gradient: vec![
                    Color32::from_hex("#5E81AC").unwrap(),
                    Color32::from_hex("#88C0D0").unwrap(),
                    Color32::from_hex("#EBCB8B").unwrap(),
                ],
                edge_gradient: vec![
                    Color32::from_hex("#164e85ff").unwrap(),
                    Color32::from_hex("#A3BE8C").unwrap(),
                ],
                cadence_gradient: vec![
                    Color32::from_hex("#164e85ff").unwrap(),
                    Color32::from_hex("#BF616A").unwrap(),
                ],
            },
        }
    }

    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".into(),
            white: Color32::from_hex("#FFFFFF").unwrap(),
            black: Color32::from_hex("#6677CC").unwrap(),
            override_color: Color32::from_hex("#FFFFFF").unwrap(),
            downbeat_color: Color32::from_hex("#000000").unwrap(),
            strong_color: Color32::from_hex("#1B1F4A").unwrap(),
            weak_color: Color32::from_hex("#3F4A8A").unwrap(),
            off_color: Color32::from_hex("#6677CC").unwrap(),
            ui: Some(UITheme {
                extreme_bg_color: Color32::from_hex("#000000").unwrap(),
                panel_fill: Color32::from_hex("#000000").unwrap(),
                window_fill: Color32::from_hex("#000000").unwrap(),
                faint_bg_color: Color32::from_hex("#000000").unwrap(),
                override_text_color: Color32::from_hex("#FFFFFF").unwrap(),
                hyperlink_color: Color32::from_hex("#00FFFF").unwrap(),
                selection_bg: Color32::from_hex("#FF5050").unwrap(),
                selection_stroke: Color32::from_hex("#000000").unwrap(),
                hovered_bg: Color32::from_hex("#000000").unwrap(),
                active_bg: Color32::from_hex("#000000").unwrap(),
                open_bg: Color32::from_hex("#000000").unwrap(),
            }),
            graph: GraphTheme {
                background: Color32::from_hex("#000000").unwrap(),
                node_gradient: vec![
                    Color32::from_hex("#FFFF00").unwrap(),
                    Color32::from_hex("#00FFFF").unwrap(),
                    Color32::from_hex("#FF00FF").unwrap(),
                ],
                edge_gradient: vec![
                    Color32::from_hex("#FFFFFF").unwrap(),
                    Color32::from_hex("#FF5050").unwrap(),
                ],
                cadence_gradient: vec![
                    Color32::from_hex("#FF0000").unwrap(),
                    Color32::from_hex("#FFFF00").unwrap(),
                ],
            },
        }
    }
}

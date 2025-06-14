use eframe::egui::{self, Color32, CornerRadius, Stroke};

use crate::app::types::ColorScheme;

impl ColorScheme {
    pub fn light() -> Self {
        Self {
            name: "Light".to_owned(),
            accent_color: "#D68C60".to_owned(),
            override_color: "#C8D4F0".to_owned(),
            downbeat_color: "#A0A0E0".to_owned(),
            strong_color: "#555555".to_owned(),
            weak_color: "#888888".to_owned(),
            off_color: "#CCCCCC".to_owned(),
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "Dark".to_owned(),
            accent_color: "#A07050".to_owned(),
            override_color: "#3C4664".to_owned(),
            downbeat_color: "#505078".to_owned(),
            strong_color: "#828282".to_owned(),
            weak_color: "#505050".to_owned(),
            off_color: "#282828".to_owned(),
        }
    }

    pub fn pastel() -> Self {
        Self {
            name: "Pastel".to_owned(),
            accent_color: "#A07050".to_owned(),
            override_color: "#3C4664".to_owned(),
            downbeat_color: "#cce2a6".to_owned(),
            strong_color: "#a3d3e0".to_owned(),
            weak_color: "#a0abde".to_owned(),
            off_color: "#8b86c6".to_owned(),
        }
    }

    pub fn black() -> Self {
        Self {
            name: "Black".to_owned(),
            accent_color: "#202020".to_owned(),
            override_color: "#000000".to_owned(),
            downbeat_color: "#404040".to_owned(),
            strong_color: "#606060".to_owned(),
            weak_color: "#808080".to_owned(),
            off_color: "#101010".to_owned(),
        }
    }
}

impl ColorScheme {
    pub fn apply_to_ctx(&self, ctx: &egui::Context) {
        match self.name.as_str() {
            "Light" => {
                ctx.set_visuals(egui::Visuals::light());
            }
            "Dark" => {
                ctx.set_visuals(egui::Visuals::dark());
            }
            "Pastel" => {
                let mut visuals = egui::Visuals::light();

                // 🍦 Base background tones
                let background = Color32::from_rgb(255, 247, 234); // warm cream
                let panel = Color32::from_rgb(250, 230, 215); // soft peach-beige
                let window = Color32::from_rgb(242, 220, 204); // almond-tan

                // ✍️ Text colors
                let text_main = Color32::from_rgb(80, 60, 50); // warm brown (not gray or purple)
                let heading = Color32::from_rgb(60, 40, 30); // dark coffee brown

                // 🎯 Accent & selection
                let accent = Color32::from_rgb(200, 225, 180); // pastel green-mint
                let accent_sub = Color32::from_rgb(180, 200, 160); // soft olive

                let selection_fill = Color32::from_rgb(240, 200, 170); // pastel apricot
                let selection_border = Color32::from_rgb(200, 170, 140); // soft burnt peach

                // 🎛 Button shades (creamier now)
                let button_idle = Color32::from_rgb(245, 222, 200); // peach cream
                let button_hover = Color32::from_rgb(240, 210, 180); // sand
                let button_active = Color32::from_rgb(230, 190, 160); // soft clay

                // 🌸 Additional accent tones (still soft, but distinct)
                let coral = Color32::from_rgb(255, 180, 170); // soft coral
                let soft_blue = Color32::from_rgb(180, 210, 230); // sky pastel
                let dusty_rose = Color32::from_rgb(220, 180, 190); // rose blush
                let mellow_yellow = Color32::from_rgb(255, 240, 180); // buttery highlight

                // ☁️ Soft shadow
                visuals.window_shadow = eframe::epaint::Shadow {
                    color: Color32::from_rgba_unmultiplied(80, 60, 40, 20),
                    offset: [0, 0],
                    blur: 5,
                    spread: 5,
                };
                visuals.popup_shadow = visuals.window_shadow;
                visuals.menu_corner_radius = CornerRadius::same(6);

                // Assign base colors
                visuals.extreme_bg_color = background;
                visuals.panel_fill = panel;
                visuals.window_fill = window;
                visuals.faint_bg_color = background;
                visuals.override_text_color = Some(text_main);

                // Selection (text, editable fields)
                visuals.selection.bg_fill = selection_fill;
                visuals.selection.stroke = Stroke::new(1.0, selection_border);

                // Widgets: soft, rounded, low-contrast
                for widget in [
                    &mut visuals.widgets.inactive,
                    &mut visuals.widgets.hovered,
                    &mut visuals.widgets.active,
                    &mut visuals.widgets.open,
                    &mut visuals.widgets.noninteractive,
                ] {
                    widget.bg_fill = button_idle;
                    widget.weak_bg_fill = button_idle;
                    widget.fg_stroke = Stroke::new(1.0, text_main);
                    widget.bg_stroke = Stroke::new(1.0, button_hover);
                }

                // Optional: tweak individual states for better polish
                visuals.widgets.hovered.bg_fill = button_hover;
                visuals.widgets.active.bg_fill = button_active;
                visuals.widgets.inactive.bg_stroke = Stroke::new(0.0, button_hover);
                visuals.widgets.noninteractive.bg_fill = panel;
                visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, heading);

                // Optional: links, sliders, etc.
                visuals.hyperlink_color = accent;
                visuals.slider_trailing_fill = false;
                // visuals.slider_rail = Stroke::new(1.0, accent_sub);
                // visuals

                visuals.widgets.inactive.bg_fill = button_idle;
                visuals.widgets.hovered.bg_fill = coral; // coral for hover
                visuals.widgets.active.bg_fill = soft_blue; // soft blue for click
                visuals.widgets.open.bg_fill = dusty_rose; // for dropdowns, collapsibles

                // ✨ Update strokes and text
                visuals.widgets.hovered.fg_stroke.color = heading;
                visuals.widgets.active.fg_stroke.color = Color32::BLACK;
                visuals.widgets.open.fg_stroke.color = heading;

                visuals.widgets.inactive.bg_stroke = Stroke::new(0.5, mellow_yellow);
                visuals.widgets.hovered.bg_stroke = Stroke::new(1.2, coral);
                visuals.widgets.active.bg_stroke = Stroke::new(1.0, soft_blue);
                visuals.widgets.open.bg_stroke = Stroke::new(1.0, dusty_rose);

                // 🔘 Slider/Progress styling
                visuals.slider_trailing_fill = true;
                visuals.selection.bg_fill = soft_blue;
                visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(120, 160, 200));

                // 🔗 Hyperlinks & interactive extras
                visuals.hyperlink_color = coral;

                // 🖼 Optional: Use mellow yellow for frame borders or window borders
                visuals.window_stroke = Stroke::new(1.0, mellow_yellow);

                ctx.set_visuals(visuals);
            }

            "Black" => {
                let mut visuals = egui::Visuals::dark();
                visuals.extreme_bg_color = egui::Color32::from_rgb(0, 0, 0);
                visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(10, 10, 10);
                visuals.window_fill = egui::Color32::from_rgb(5, 5, 5);
                visuals.override_text_color = Some(egui::Color32::from_gray(200));
                ctx.set_visuals(visuals);
            }
            _ => {}
        }
    }
}

use eframe::egui::{self, Color32, CornerRadius, Stroke};

use crate::app::types::ColorScheme;

impl ColorScheme {
    pub fn light() -> Self {
        Self {
            name: "Light".to_owned(),
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
            override_color: "#3C4664".to_owned(),
            downbeat_color: "#cce2a6".to_owned(),
            strong_color: "#a3d3e0".to_owned(),
            weak_color: "#a0abde".to_owned(),
            off_color: "#8b86c6".to_owned(),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".to_owned(),
            override_color: "#D8DEE9".to_owned(), // Frost text (light gray-blue)
            downbeat_color: "#88C0D0".to_owned(), // Frost blue
            strong_color: "#81A1C1".to_owned(),   // Muted sky blue
            weak_color: "#5E81AC".to_owned(),     // Desaturated steel blue
            off_color: "#2E3440".to_owned(),      // Polar night (near black)
        }
    }

    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_owned(),
            override_color: "#FFFFFF".to_owned(), // text only
            off_color: "#000000".to_owned(),      // black
            weak_color: "#1B1F4A".to_owned(),     // very dark navy
            strong_color: "#3F4A8A".to_owned(),   // medium indigo
            downbeat_color: "#6677CC".to_owned(), // bright but distinct
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

                // 🎛 Button shades (creamier now)
                let button_idle = Color32::from_rgb(245, 222, 200); // peach cream
                let button_hover = Color32::from_rgb(240, 210, 180); // sand

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
                visuals.widgets.inactive.bg_stroke = Stroke::new(0.0, button_hover);
                visuals.widgets.noninteractive.bg_fill = panel;
                visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, heading);

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

            "Nord" => {
                let mut visuals = egui::Visuals::light();

                // 🧊 Nord base colors
                let nord0 = Color32::from_rgb(46, 52, 64); // background
                let nord1 = Color32::from_rgb(59, 66, 82); // panel/window fill
                let nord2 = Color32::from_rgb(67, 76, 94); // slightly lighter background
                let nord3 = Color32::from_rgb(76, 86, 106); // faint_bg_color, noninteractive

                let nord4 = Color32::from_rgb(216, 222, 233); // main text
                let nord5 = Color32::from_rgb(229, 233, 240); // heading
                let nord6 = Color32::from_rgb(136, 192, 208); // slider/active fill
                let nord7 = Color32::from_rgb(163, 190, 140); // hover accent
                let nord8 = Color32::from_rgb(235, 203, 139); // warning highlight
                let nord9 = Color32::from_rgb(191, 97, 106); // selection/red
                let nord10 = Color32::from_rgb(180, 142, 173); // purple for open

                // Shadows
                visuals.popup_shadow = visuals.window_shadow;

                // Base layout
                visuals.extreme_bg_color = nord0;
                visuals.panel_fill = nord1;
                visuals.window_fill = nord2;
                visuals.faint_bg_color = nord3;
                visuals.override_text_color = Some(nord4);

                // Widgets
                for widget in [
                    &mut visuals.widgets.inactive,
                    &mut visuals.widgets.hovered,
                    &mut visuals.widgets.active,
                    &mut visuals.widgets.open,
                    &mut visuals.widgets.noninteractive,
                ] {
                    widget.bg_fill = nord1;
                    widget.weak_bg_fill = nord2;
                    widget.fg_stroke = Stroke::new(1.0, nord4);
                    widget.bg_stroke = Stroke::new(1.0, nord3);
                }

                // Tweaks
                visuals.widgets.hovered.bg_fill = nord7;
                visuals.widgets.active.bg_fill = nord6;
                visuals.widgets.open.bg_fill = nord10;

                visuals.widgets.hovered.fg_stroke.color = nord5;
                visuals.widgets.active.fg_stroke.color = nord0;
                visuals.widgets.open.fg_stroke.color = nord4;

                visuals.widgets.hovered.bg_stroke = Stroke::new(1.2, nord7);
                visuals.widgets.active.bg_stroke = Stroke::new(1.2, nord6);
                visuals.widgets.open.bg_stroke = Stroke::new(1.0, nord10);

                visuals.widgets.noninteractive.bg_fill = nord3;
                visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, nord5);

                // Selection, sliders, links
                visuals.selection.bg_fill = nord9;
                visuals.selection.stroke = Stroke::new(1.0, nord3);
                visuals.slider_trailing_fill = true;
                visuals.hyperlink_color = nord8;
                visuals.window_stroke = Stroke::new(1.0, nord3);

                ctx.set_visuals(visuals);
            }

            "High Contrast" => {
                let mut visuals = egui::Visuals::dark();

                // 🎨 Colors for high-contrast mode
                let black = Color32::BLACK;
                let white = Color32::WHITE;
                let yellow = Color32::from_rgb(255, 255, 0);
                let cyan = Color32::from_rgb(0, 255, 255);
                let magenta = Color32::from_rgb(255, 0, 255);
                let bright_red: Color32 = Color32::from_rgb(255, 80, 80);

                // Backgrounds
                visuals.extreme_bg_color = black;
                visuals.panel_fill = black;
                visuals.window_fill = black;
                visuals.faint_bg_color = black;

                // Text & hyperlink color
                visuals.override_text_color = Some(white);
                visuals.hyperlink_color = cyan;

                // 🔳 Widget base: strong border, flat background
                for widget in [
                    &mut visuals.widgets.inactive,
                    &mut visuals.widgets.hovered,
                    &mut visuals.widgets.active,
                    &mut visuals.widgets.open,
                    &mut visuals.widgets.noninteractive,
                ] {
                    widget.bg_fill = black;
                    widget.weak_bg_fill = black;
                    widget.fg_stroke = Stroke::new(2.0, white); // Text or icon color
                    widget.bg_stroke = Stroke::new(2.0, white); // Border
                    widget.corner_radius = CornerRadius::same(0); // Square corners
                }

                // 🔁 States: extremely distinct colors
                visuals.widgets.hovered.bg_fill = black;
                visuals.widgets.hovered.bg_stroke = Stroke::new(2.0, yellow);
                visuals.widgets.hovered.fg_stroke = Stroke::new(2.0, yellow);

                visuals.widgets.active.bg_fill = black;
                visuals.widgets.active.bg_stroke = Stroke::new(2.0, cyan);
                visuals.widgets.active.fg_stroke = Stroke::new(2.0, cyan);

                visuals.widgets.open.bg_fill = black;
                visuals.widgets.open.bg_stroke = Stroke::new(2.0, magenta);
                visuals.widgets.open.fg_stroke = Stroke::new(2.0, magenta);

                visuals.widgets.noninteractive.bg_fill = black;
                visuals.widgets.noninteractive.fg_stroke = Stroke::new(2.0, white);

                // 📊 Slider/selection
                visuals.slider_trailing_fill = true;
                visuals.selection.bg_fill = bright_red;
                visuals.selection.stroke = Stroke::new(2.0, black);

                // Outlines & window
                visuals.window_stroke = Stroke::new(2.0, white);
                visuals.window_shadow = eframe::epaint::Shadow::NONE;
                visuals.popup_shadow = eframe::epaint::Shadow::NONE;
                visuals.menu_corner_radius = CornerRadius::same(0);

                ctx.set_visuals(visuals);
            }
            _ => {}
        }
    }
}

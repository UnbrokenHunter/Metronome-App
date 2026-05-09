use crate::app::systems::colors::theme_presets::Theme;
use egui::{Color32, CornerRadius, Stroke};

impl Theme {
    pub fn apply_to_ctx(&self, ctx: &egui::Context) {
        match self.name.as_str() {
            "Light" => {
                ctx.set_visuals(egui::Visuals::light());
            }
            "Dark" => {
                ctx.set_visuals(egui::Visuals::dark());
            }
            _ => {
                let mut visuals = egui::Visuals::light();

                let Some(colors) = &self.ui else {
                    return;
                };

                // Base visuals
                visuals.extreme_bg_color = colors.extreme_bg_color;
                visuals.panel_fill = colors.panel_fill;
                visuals.window_fill = colors.window_fill;
                visuals.faint_bg_color = colors.faint_bg_color;
                visuals.override_text_color = Some(colors.override_text_color);
                visuals.hyperlink_color = colors.hyperlink_color;

                visuals.selection.bg_fill = colors.selection_bg;
                visuals.selection.stroke = Stroke::new(1.0, colors.selection_stroke);
                visuals.slider_trailing_fill = true;

                visuals.window_stroke = Stroke::new(1.0, colors.selection_stroke);
                visuals.window_shadow = eframe::epaint::Shadow {
                    color: Color32::from_rgba_unmultiplied(80, 60, 40, 20),
                    offset: [0, 0],
                    blur: 5,
                    spread: 5,
                };
                visuals.popup_shadow = visuals.window_shadow;
                visuals.menu_corner_radius = CornerRadius::same(6);

                for widget in [
                    &mut visuals.widgets.inactive,
                    &mut visuals.widgets.hovered,
                    &mut visuals.widgets.active,
                    &mut visuals.widgets.open,
                    &mut visuals.widgets.noninteractive,
                ] {
                    widget.bg_fill = colors.panel_fill;
                    widget.weak_bg_fill = colors.window_fill;
                    widget.fg_stroke = Stroke::new(1.0, colors.override_text_color);
                    widget.bg_stroke = Stroke::new(1.0, colors.selection_stroke);
                }

                // Specialized widget states using new colors
                visuals.widgets.hovered.bg_fill = colors.hovered_bg;
                visuals.widgets.active.bg_fill = colors.active_bg;
                visuals.widgets.open.bg_fill = colors.open_bg;

                visuals.widgets.hovered.fg_stroke.color = colors.override_text_color;
                visuals.widgets.active.fg_stroke.color = colors.override_text_color;
                visuals.widgets.open.fg_stroke.color = colors.override_text_color;

                visuals.widgets.hovered.bg_stroke = Stroke::new(1.2, colors.selection_stroke);
                visuals.widgets.active.bg_stroke = Stroke::new(1.0, colors.selection_stroke);
                visuals.widgets.open.bg_stroke = Stroke::new(1.0, colors.selection_stroke);

                visuals.widgets.noninteractive.bg_fill = colors.faint_bg_color;
                visuals.widgets.noninteractive.fg_stroke =
                    Stroke::new(1.0, colors.override_text_color);

                ctx.set_visuals(visuals);
            }
        }
    }
}

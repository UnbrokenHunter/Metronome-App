use super::section::{full_width_button, settings_section};
use crate::app::systems::colors::themes::{all_themes, select_theme, theme};
use crate::app::AppData;
use eframe::egui::{self, RichText, ScrollArea, Ui};

pub(crate) fn settings_general(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            ui.label(RichText::new("Settings").size(45.0));
            ui.separator();

            settings_section(
                ui,
                "Theme",
                "The color scheme that will be used for the UI.",
                |ui| theme_selector(ui),
            );

            settings_section(
                ui,
                "Reset All Presets",
                "Resets all Accent Menu Presets to just the defaults.",
                |ui| {
                    if full_width_button(ui, "Reset") {
                        app.accent_presets = AppData::load_default_accent_presets();
                    }
                },
            );

            settings_section(
                ui,
                "Reset All Settings",
                "Resets everything to its default state.",
                |ui| {
                    if full_width_button(ui, "Reset") {
                        app.settings = AppData::load_default_settings();
                    }
                },
            );
        });
    });
}

fn theme_selector(ui: &mut Ui) {
    let theme_options = all_themes();
    let s = theme(ui.ctx());
    let current_theme = theme_options.iter().enumerate().find(|(_, f)| **f == s);

    if let Some((current_index, current_theme)) = current_theme {
        egui::ComboBox::from_label("Theme")
            .selected_text(&current_theme.name)
            .show_ui(ui, |ui| {
                for (i, option) in theme_options.iter().enumerate() {
                    let is_selected = current_index == i;

                    if ui
                        .selectable_label(is_selected, option.name.clone())
                        .clicked()
                    {
                        let _ = select_theme(ui.ctx(), i);
                        theme(ui.ctx()).apply_to_ctx(ui.ctx());
                    }
                }
            });
    }
}

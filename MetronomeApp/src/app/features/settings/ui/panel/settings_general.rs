use super::section::{full_width_button, settings_section};
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
                |ui| theme_selector(app, ui),
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
                        app.current_theme().apply_to_ctx(ui.ctx()); // reapply the theme
                    }
                },
            );
        });
    });
}

fn theme_selector(app: &mut AppData, ui: &mut Ui) {
    let theme_options = app.themes.all();

    let current_index = app
        .settings
        .selected_theme_index
        .min(theme_options.len().saturating_sub(1));

    let current_theme = &theme_options[current_index];

    let mut selected_index: Option<usize> = None;

    egui::ComboBox::from_label("Theme")
        .selected_text(&current_theme.name)
        .show_ui(ui, |ui| {
            for (i, option) in theme_options.iter().enumerate() {
                let is_selected = current_index == i;

                if ui.selectable_label(is_selected, &option.name).clicked() {
                    selected_index = Some(i);
                }
            }
        });

    if let Some(i) = selected_index {
        app.settings.selected_theme_index = i;
        app.current_theme().apply_to_ctx(ui.ctx());
    }
}
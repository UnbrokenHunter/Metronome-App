use crate::app::{AppData, ColorScheme};
use eframe::egui::{self, RichText, ScrollArea, Ui};

use super::section::{full_width_button, settings_section};

const THEME_OPTIONS: [&str; 5] = ["Light", "Dark", "Pastel", "Nord", "High Contrast"];

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
                    }
                },
            );
        });
    });
}

fn theme_selector(app: &mut AppData, ui: &mut Ui) {
    egui::ComboBox::from_label("Theme")
        .selected_text(&app.settings.color_scheme.name)
        .show_ui(ui, |ui| {
            for option in THEME_OPTIONS {
                let is_selected = app.settings.color_scheme.name == option;

                if ui.selectable_label(is_selected, option).clicked() {
                    let new_scheme = color_scheme_from_name(option);
                    new_scheme.apply_to_ctx(ui.ctx());
                    app.settings.color_scheme = new_scheme;
                }
            }
        });
}

fn color_scheme_from_name(name: &str) -> ColorScheme {
    match name {
        "Light" => ColorScheme::light(),
        "Dark" => ColorScheme::dark(),
        "Pastel" => ColorScheme::pastel(),
        "Nord" => ColorScheme::nord(),
        "High Contrast" => ColorScheme::high_contrast(),
        _ => ColorScheme::dark(),
    }
}

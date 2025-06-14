use eframe::egui::{self, RichText, ScrollArea, Ui};

use crate::app::{AppData, types::ColorScheme};

pub fn general_settings_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            ui.label(RichText::new("Settings").size(45.0));
            ui.separator();

            let theme_options = ["Light", "Dark", "Pastel", "High Contrast"];
            let current_theme = &app.settings.color_scheme.name;

            egui::ComboBox::from_label("Theme")
                .selected_text(current_theme)
                .show_ui(ui, |ui| {
                    for &option in &theme_options {
                        let is_selected = app.settings.color_scheme.name == option;

                        if ui.selectable_label(is_selected, option).clicked() {
                            let new_scheme = match option {
                                "Light" => ColorScheme::light(),
                                "Dark" => ColorScheme::dark(),
                                "Pastel" => ColorScheme::pastel(),
                                "High Contrast" => ColorScheme::high_contrast(),
                                _ => ColorScheme::dark(),
                            };

                            new_scheme.apply_to_ctx(ui.ctx());
                            app.settings.color_scheme = new_scheme;
                        }
                    }
                });

            ui.heading("Reset All Settings");
            if ui
                .add_sized([ui.available_width(), 30.0], egui::Button::new("Reset"))
                .clicked()
            {
                app.reset_settings();
            }
        });
    });
}

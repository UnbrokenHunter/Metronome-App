use crate::app::AppData;
use eframe::egui::{self, Align, ComboBox, Layout, TextEdit, TextStyle, Ui};

use super::utils::{icon_button, truncate, SAVE_ICON_SIZE};

pub fn draw_accent_top_bar(app: &mut AppData, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if icon_button(
            ui,
            egui::include_image!("../../../../../assets/icons/save.png"),
            SAVE_ICON_SIZE,
        ) {
            app.accent_presets
                .accent_chains
                .push(app.parameters.accents.clone());
        }

        let available_width = ui.available_width();

        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            let dropdown_width = 180.0;
            let header_width = available_width - dropdown_width;

            ui.add_sized(
                [header_width.max(0.0), 27.0],
                TextEdit::singleline(&mut app.parameters.accents.name)
                    .font(TextStyle::Heading)
                    .hint_text("Title..."),
            );

            let mut selected_preset_index = None;

            ComboBox::from_label("")
                .selected_text(truncate(&app.parameters.accents.name, 22))
                .height(300.0)
                .show_ui(ui, |ui| {
                    for (index, preset) in app.accent_presets.accent_chains.iter().enumerate() {
                        if ui
                            .selectable_label(
                                app.parameters.accents.name == preset.name,
                                &preset.name,
                            )
                            .clicked()
                        {
                            selected_preset_index = Some(index);
                        }
                    }
                });

            if let Some(index) = selected_preset_index
                && let Some(preset) = app.accent_presets.accent_chains.get(index)
            {
                app.parameters.accents = preset.clone();
            }
        });
    });
}
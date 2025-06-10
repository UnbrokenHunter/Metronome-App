use crate::app::{AppData, Menus};
use eframe::egui::{self, Context, Ui};

pub fn tabs_layout(app: &mut AppData, ctx: &Context) {
    egui::TopBottomPanel::top("tabs")
        .resizable(false)
        .show(ctx, |ui| {
            // egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui: &mut Ui| {
                let tab_size = [ui.available_width() / 10.0, 25.0];

                // Open Metronome
                if ui
                    .add_sized(tab_size, egui::Button::new("Metronome"))
                    .clicked()
                {
                    change_menus(app, Menus::Metronome);
                }
                // Open Accents
                if ui
                    .add_sized(tab_size, egui::Button::new("Accents"))
                    .clicked()
                {
                    change_menus(app, Menus::Accents);
                }

                // Open Logs Button
                if ui.add_sized(tab_size, egui::Button::new("Logs")).clicked() {
                    change_menus(app, Menus::Logs);
                }
                // Open Settings Button
                if ui
                    .add_sized(tab_size, egui::Button::new("Settings"))
                    .clicked()
                {
                    change_menus(app, Menus::Settings);
                }
            });
        });
    // });
}

fn change_menus(app: &mut AppData, menu: Menus) {
    app.runtime.menu_state = 0;
    app.runtime.menu = menu;
}

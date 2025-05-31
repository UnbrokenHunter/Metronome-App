use crate::app::ui::settings;
use crate::app::{MyApp, logic};
use eframe::egui::Ui;

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    settings::practice_ui(app, ui);
    settings::tempo_ui(app, ui);

    settings::growth_ui(app, ui);
    settings::sound_ui(app, ui);
}

pub fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        logic::tempo::calculate_tempo(app);

        settings::plot_ui(app, ui);

        ui.horizontal(|ui: &mut Ui| {
            settings::play_ui(app, ui);
            settings::info_ui(app, ui);
        });
    });
}

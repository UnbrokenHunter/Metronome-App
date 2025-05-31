use crate::app::ui::settings;
use crate::app::{MyApp, logic};
use eframe::egui::{self, Context, Ui};

pub fn layout(app: &mut MyApp, ctx: &Context) {
    egui::SidePanel::left("settings")
        .resizable(false)
        .show(ctx, |ui| {
            settings_ui(app, ui);
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        main_ui(app, ui);
    });
}

fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    if !app.playing {
        settings::practice_ui(app, ui);
        settings::tempo_ui(app, ui);

        settings::growth_ui(app, ui);
        settings::sound_ui(app, ui);
    }
}

fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        logic::tempo::calculate_tempo(app);

        settings::plot_ui(app, ui);

        ui.horizontal(|ui: &mut Ui| {
            settings::play_ui(app, ui);
            settings::info_ui(app, ui);
        });
    });
}

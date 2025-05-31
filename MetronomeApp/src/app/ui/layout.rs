use crate::app::ui::settings;
use crate::app::{MyApp, logic};
use eframe::egui::{self, Context, RichText, ScrollArea, Ui};

pub fn layout(app: &mut MyApp, ctx: &Context) {
    settings_ui(app, ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        main_ui(app, ui);
    });
}

fn settings_ui(app: &mut MyApp, ctx: &Context) {
    egui::SidePanel::left("settings")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("BPM:").size(45.0));
                    ui.label(RichText::new(format!("{:.2} BPM", app.tempo)).size(45.0));
                });
                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    settings::practice_ui(app, ui);
                    settings::tempo_ui(app, ui);

                    settings::growth_ui(app, ui);
                    settings::sound_ui(app, ui);
                });
            });
        });
}

fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        logic::tempo::calculate_tempo(app);

        settings::parameters_ui(app, ui);
        settings::plot_ui(app, ui);
        ui.horizontal(|ui: &mut Ui| {
            settings::play_ui(app, ui);
            settings::info_ui(app, ui);
        });
    });
}

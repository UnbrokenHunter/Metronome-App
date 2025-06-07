use crate::app::ui::settings;
use crate::app::{AppData, logic};
use eframe::egui::{self, Context, RichText, ScrollArea, Ui};

pub fn layout(app: &mut AppData, ctx: &Context) {
    header_ui(app, ctx);
    settings_ui(app, ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        main_ui(app, ui);
    });
}

fn header_ui(app: &mut AppData, ctx: &Context) {
    egui::TopBottomPanel::top("tabs")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                // Open Logs Button
                if ui
                    .add_sized([ui.available_width(), 30.0], egui::Button::new("Open Logs"))
                    .clicked()
                {
                    app.runtime.playing = !app.runtime.playing;
                }
            });
        });
}

fn settings_ui(app: &mut AppData, ctx: &Context) {
    egui::SidePanel::left("settings")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(RichText::new(format!("BPM: {:.2} BPM", app.runtime.tempo)).size(45.0));
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

fn main_ui(app: &mut AppData, ui: &mut Ui) {
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

use eframe::egui::{self, RichText, ScrollArea, Ui};

use crate::app::AppData;

pub fn log_settings_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ScrollArea::vertical().show(ui, |ui| {

        ui.label(RichText::new("Log Settings").size(45.0));
        ui.separator();


        // Save Log Data
        ui.heading("Save Log Data");
        let label = if app.settings.save_logs {
            "Will Save"
        } else {
            "Will Not Save"
        };

        ui.toggle_value(&mut app.settings.save_logs, label);
        ui.label(
            egui::RichText::new("Whether or not information such as practice session time, duration, and metrics such as average tempo, etc should be saved. If it is saved, it can be viewed in the logs tab of the program.")
                .small()
                .color(egui::Color32::GRAY),
        );
        ui.separator();
        ui.add_space(8.0);

        // Minimum Practice Length
                ui.heading("Minimum Practice Length");
            ui.add(
                egui::Slider::new(&mut app.settings.min_practice_length, 0..=6_0000).text("Miliseconds"),
            );
        ui.label(
            egui::RichText::new("How long must a practice be in order to be saved as a log. This is to prevent accidental clicks of of the play button from clogging up the logs data.")
                .small()
                .color(egui::Color32::GRAY),
        );
        ui.separator();
        ui.add_space(8.0);


        // Save Plots
        ui.heading("Save Plots");
        let label = if app.settings.save_plots {
            "Will Save"
        } else {
            "Will Not Save"
        };
        ui.toggle_value(&mut app.settings.save_plots, label);
        ui.label(
            egui::RichText::new("Whether or not a practice sessions tempo plot should be saved. The tempo plot will give the most accurate picture of the session, however it compared to other metrics, it will require more storage on the computer.")
                .small()
                .color(egui::Color32::GRAY),
        );
                ui.separator();
        ui.add_space(8.0);


        // Plot Granulatrity
        // Only Show Plot Granularity if they are being saved in the first place
        if app.settings.save_plots {
            ui.heading("Plot Granularity");
            ui.radio_value(&mut app.settings.plot_granularity, 0, "Low");
            ui.radio_value(&mut app.settings.plot_granularity, 1, "Medium");
            ui.radio_value(&mut app.settings.plot_granularity, 2, "High");
            ui.radio_value(&mut app.settings.plot_granularity, 3, "Lossless");
                    ui.label(
            egui::RichText::new("How high resolution should the plots be saved at. A Low resolution will result in less storage, but larger gaps between points saved. This means that the data will be less precice. A High granularity will save most of the points, resulting in greater accuracy, but also greater storage requirements. Lossless will save every point.")
                .small()
                .color(egui::Color32::GRAY),
        );
                ui.separator();
ui.add_space(8.0);

        }

        ui.heading("Delete Logs");
        if ui
            .add_sized([ui.available_width(), 30.0], egui::Button::new("Delete"))
            .clicked()
        {
            app.practice.logs.clear();
        }
    });
    });
}

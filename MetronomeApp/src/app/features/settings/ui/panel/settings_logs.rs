use crate::app::AppData;
use eframe::egui::{self, RichText, ScrollArea, Ui};

use super::section::{full_width_button, settings_section};

pub(crate) fn settings_logs(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            ui.label(RichText::new("Log Settings").size(45.0));
            ui.separator();

            settings_section(
                ui,
                "Save Log Data",
                "Whether practice session time, duration, average tempo, and other metrics should be saved. If saved, they can be viewed in the logs tab.",
                |ui| {
                    let label = if app.settings.save_logs {
                        "Will Save"
                    } else {
                        "Will Not Save"
                    };

                    ui.toggle_value(&mut app.settings.save_logs, label);
                },
            );

            settings_section(
                ui,
                "Request Log Title on Save",
                "If enabled, a popup requesting a title will appear on every log save. This can be used to quickly label what was practiced during the session.",
                |ui| {
                    let label = if app.settings.do_title_popup {
                        "Popup Enabled"
                    } else {
                        "Popup Disabled"
                    };

                    ui.toggle_value(&mut app.settings.do_title_popup, label);
                },
            );

            settings_section(
                ui,
                "Minimum Practice Length",
                "How long a practice must be before it is saved as a log. This prevents accidental play-button clicks from clogging up the logs.",
                |ui| {
                    ui.add(
                        egui::Slider::new(&mut app.settings.min_practice_length, 0..=60_000)
                            .text("Milliseconds"),
                    );
                },
            );

            settings_section(
                ui,
                "Save Plots",
                "Whether a practice session's tempo plot should be saved. Plots give the most accurate picture of a session, but require more storage.",
                |ui| {
                    let label = if app.settings.save_plots {
                        "Will Save"
                    } else {
                        "Will Not Save"
                    };

                    ui.toggle_value(&mut app.settings.save_plots, label);
                },
            );

            if app.settings.save_plots {
                settings_section(
                    ui,
                    "Plot Granularity",
                    "How high-resolution saved plots should be. Low saves less data but is less precise. Lossless saves every point.",
                    |ui| {
                        ui.radio_value(&mut app.settings.plot_granularity, 0, "Low");
                        ui.radio_value(&mut app.settings.plot_granularity, 1, "Medium");
                        ui.radio_value(&mut app.settings.plot_granularity, 2, "High");
                        ui.radio_value(&mut app.settings.plot_granularity, 3, "Lossless");
                    },
                );
            }

            ui.heading("Delete Logs");

            if full_width_button(ui, "Delete") {
                app.practice.logs.clear();
            }
        });
    });
}

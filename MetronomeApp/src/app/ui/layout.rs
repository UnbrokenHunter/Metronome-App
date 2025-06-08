use crate::app::ui::{general, graph, parameters};
use crate::app::{AppData, Menus, logic};
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
                    .add_sized([ui.available_width(), 30.0], egui::Button::new("Logs"))
                    .clicked()
                {
                    app.runtime.menu = Menus::Logs;
                }
                // Open Metronome
                if ui
                    .add_sized([ui.available_width(), 30.0], egui::Button::new("Metronome"))
                    .clicked()
                {
                    app.runtime.menu = Menus::Metronome;
                }
            });
        });
}

fn settings_ui(app: &mut AppData, ctx: &Context) {
    egui::SidePanel::left("settings")
        .resizable(false)
        .show(ctx, |ui| {
            if app.runtime.menu == Menus::Metronome {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("BPM: {:.2} BPM", app.runtime.tempo)).size(45.0),
                    );
                    ui.separator();

                    ScrollArea::vertical().show(ui, |ui| {
                        parameters::practice_ui(app, ui);
                        parameters::tempo_ui(app, ui);

                        parameters::growth_ui(app, ui);
                        parameters::sound_ui(app, ui);
                    });
                });
            } else if app.runtime.menu == Menus::Logs {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    for (i, log) in app.practice.logs.iter().enumerate() {
                        // First draw the content inside a frame
                        let response = egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.allocate_ui_with_layout(
                                egui::vec2(ui.available_width(), 0.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.heading(format!(
                                        "{}",
                                        logic::clock::format_date(log.time_started, None)
                                    ));
                                    ui.separator();
                                    ui.label(format!(
                                        "Duration: {}",
                                        logic::clock::format_time(log.duration_ms as u128)
                                    ));
                                    ui.label(format!("Average Tempo: {}", log.average_tempo));
                                    ui.label(format!("Average Delta: {}", log.average_delta));
                                },
                            );
                        });

                        // Make the whole frame area clickable
                        let clicked = ui
                            .interact(
                                response.response.rect,
                                egui::Id::new(format!("log_{}", i)),
                                egui::Sense::click(),
                            )
                            .clicked();

                        if clicked {
                            println!("Clicked log at index {i}");
                        }

                        ui.add_space(8.0); // Add spacing between logs
                    }
                });
            }
        });
}

fn main_ui(app: &mut AppData, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        if app.runtime.menu == Menus::Metronome {
            logic::tempo::calculate_tempo(app);

            parameters::parameters_ui(app, ui);
            graph::plot_ui(app, ui);
            ui.horizontal(|ui: &mut Ui| {
                general::play_ui(app, ui);
                general::info_ui(app, ui);
            });
        } else if app.runtime.menu == Menus::Logs {
        }
    });
}

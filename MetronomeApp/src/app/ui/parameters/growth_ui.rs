use crate::app::ui::graph::plot::draw_demo_plot;
use crate::app::{AppData, GrowthType};
use eframe::egui::{self, Ui};

pub fn growth_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Growth Behavior:");
        ui.separator();

        ui.horizontal(|ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut app.parameters.growth_type, GrowthType::Linear, "Linear");
                if ui
                    .selectable_value(
                        &mut app.parameters.growth_type,
                        GrowthType::Exponential,
                        "Exponential",
                    )
                    .clicked()
                {
                    app.parameters.tempo_params.scaler = 3.0;
                }
                if ui
                    .selectable_value(
                        &mut app.parameters.growth_type,
                        GrowthType::Sigmoidal,
                        "Sigmoidal",
                    )
                    .clicked()
                {
                    app.parameters.tempo_params.scaler = 6.0;
                }
                if ui
                    .selectable_value(
                        &mut app.parameters.growth_type,
                        GrowthType::Logarithmic,
                        "Logarithmic",
                    )
                    .clicked()
                {
                    app.parameters.tempo_params.scaler = 0.5;
                }
                if ui
                    .selectable_value(&mut app.parameters.growth_type, GrowthType::Sine, "Sine")
                    .clicked()
                {
                    app.parameters.tempo_params.scaler = 3.0;
                }

                ui.selectable_value(&mut app.parameters.growth_type, GrowthType::Constant, "Constant");
            });

            ui.vertical(|ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    draw_demo_plot(ui, app.parameters.growth_type, app.parameters.tempo_params);

                    match app.parameters.growth_type {
                        GrowthType::Exponential => {
                            ui.add(
                                egui::Slider::new(&mut app.parameters.tempo_params.scaler, 1.0..=10.0)
                                    .text("Scaler"),
                            );
                        }
                        GrowthType::Logarithmic => {
                            ui.add(
                                egui::Slider::new(&mut app.parameters.tempo_params.scaler, 0.01..=1.0)
                                    .text("Scaler"),
                            );
                        }
                        GrowthType::Sigmoidal => {
                            ui.add(
                                egui::Slider::new(&mut app.parameters.tempo_params.scaler, 1.0..=10.0)
                                    .text("Scaler"),
                            );
                        }
                        GrowthType::Sine => {
                            ui.add(
                                egui::Slider::new(&mut app.parameters.tempo_params.scaler, 1.00..=50.0)
                                    .text("Scaler"),
                            );
                        }

                        _ => {}
                    }
                });
            });
        });
    });
}

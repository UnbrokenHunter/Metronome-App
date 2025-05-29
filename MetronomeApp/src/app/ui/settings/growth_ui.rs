use crate::app::ui::plot::draw_demo_plot;
use crate::app::{GrowthType, MyApp};
use eframe::egui::{self, Ui};

pub fn growth_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Growth Behavior:");
        ui.separator();

        ui.horizontal(|ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut app.growth_type, GrowthType::Linear, "Linear");
                if ui
                    .selectable_value(&mut app.growth_type, GrowthType::Exponential, "Exponential")
                    .clicked()
                {
                    app.tempo_params.scaler = 3.0;
                }
                if ui
                    .selectable_value(&mut app.growth_type, GrowthType::Sigmoidal, "Sigmoidal")
                    .clicked()
                {
                    app.tempo_params.scaler = 6.0;
                }
                if ui
                    .selectable_value(&mut app.growth_type, GrowthType::Logarithmic, "Logarithmic")
                    .clicked()
                {
                    app.tempo_params.scaler = 0.5;
                }
                ui.selectable_value(&mut app.growth_type, GrowthType::Constant, "Constant");
            });

            ui.vertical(|ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    draw_demo_plot(ui, app.growth_type, app.tempo_params);

                    match app.growth_type {
                        GrowthType::Exponential => {
                            ui.add(
                                egui::Slider::new(&mut app.tempo_params.scaler, 1.0..=10.0)
                                    .text("Scaler"),
                            );
                        }
                        GrowthType::Logarithmic => {
                            ui.add(
                                egui::Slider::new(&mut app.tempo_params.scaler, 0.0..=1.0)
                                    .text("Scaler"),
                            );
                        }
                        GrowthType::Sigmoidal => {
                            ui.add(
                                egui::Slider::new(&mut app.tempo_params.scaler, 1.0..=10.0)
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

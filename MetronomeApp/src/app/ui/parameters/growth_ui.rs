use crate::app::ui::graph::plot::draw_demo_plot;
use crate::app::{AppData, GrowthType};
use eframe::egui::{self, Ui};

pub fn growth_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Growth Behavior:");
        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                growth_type_buttons(app, ui);
            });

            ui.vertical(|ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    draw_demo_plot(
                        ui,
                        app.parameters.growth_type,
                        app.parameters.tempo_params,
                    );

                    scaler_slider(app, ui);
                });
            });
        });
    });
}

fn growth_type_buttons(app: &mut AppData, ui: &mut Ui) {
    let options = [
        (GrowthType::Linear, "Linear", None),
        (GrowthType::Exponential, "Exponential", Some(3.0)),
        (GrowthType::Sigmoidal, "Sigmoidal", Some(6.0)),
        (GrowthType::Logarithmic, "Logarithmic", Some(0.5)),
        (GrowthType::Sine, "Sine", Some(3.0)),
        (GrowthType::Constant, "Constant", None),
    ];

    for (growth_type, label, default_scaler) in options {
        if ui
            .selectable_value(&mut app.parameters.growth_type, growth_type, label)
            .clicked()
        {
            if let Some(default_scaler) = default_scaler {
                app.parameters.tempo_params.scaler = default_scaler;
            }
        }
    }
}

fn scaler_slider(app: &mut AppData, ui: &mut Ui) {
    let scaler_range = match app.parameters.growth_type {
        GrowthType::Exponential => Some(1.0..=10.0),
        GrowthType::Logarithmic => Some(0.01..=1.0),
        GrowthType::Sigmoidal => Some(1.0..=10.0),
        GrowthType::Sine => Some(1.0..=50.0),
        _ => None,
    };

    if let Some(range) = scaler_range {
        ui.add(
            egui::Slider::new(&mut app.parameters.tempo_params.scaler, range)
                .text("Scaler"),
        );
    }
}
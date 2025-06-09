use crate::app::{AppData, GrowthType};
use eframe::egui::{self, Ui};

pub fn tempo_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Tempo:");
        ui.separator();
        if app.parameters.growth_type == GrowthType::Constant {
            ui.add(egui::Slider::new(&mut app.parameters.tempo_params.min, 1..=400).text("Tempo"));
        } else {
            ui.add(
                egui::Slider::new(&mut app.parameters.tempo_params.min, 1..=400)
                    .text("Starting Tempo"),
            );
            if ui
                .add(
                    /*_sized([ui.available_width(), 30.0], */ egui::Button::new("Tap"),
                )
                .clicked()
            {
                let last = app.runtime.last_tap_tempo_click;
                app.runtime.last_tap_tempo_click = app.runtime.time_data.time_since_start;
                println!("Last {}, Other {} ", last, app.runtime.last_tap_tempo_click);
                if last != 0 {
                    app.parameters.tempo_params.min = (1.0
                        / ((app.runtime.last_tap_tempo_click - last) as f64 / 1000.0) as f64
                        * 60.0) as u32
                }
            }
            ui.add(
                egui::Slider::new(&mut app.parameters.tempo_params.max, 1..=400)
                    .text("Ending Tempo"),
            );
        }
    });
}

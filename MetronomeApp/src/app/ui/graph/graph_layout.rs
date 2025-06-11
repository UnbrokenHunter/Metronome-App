use crate::app::ui::{general, graph, parameters};
use crate::app::{AppData, logic};
use eframe::egui::{self, Ui};

pub fn graph_layout(app: &mut AppData, ui: &mut Ui) {
    let total_width = ui.available_width();
    let plot_width = total_width * 1.0 / 2.0;
    let label_width = total_width - plot_width;

    ui.vertical_centered(|ui| {
        logic::tempo::calculate_tempo(app);

        parameters::parameters_ui(app, ui);
        graph::plot_ui(app, ui);
        ui.horizontal(|ui: &mut Ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(label_width, ui.available_height()),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    general::play_ui(app, ui);
                },
            );
            general::info_ui(app, ui);
        });
    });
}

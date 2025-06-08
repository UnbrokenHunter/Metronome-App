use crate::app::ui::{general, graph, parameters};
use crate::app::{AppData, logic};
use eframe::egui::Ui;

pub fn graph_layout(app: &mut AppData, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        logic::tempo::calculate_tempo(app);

        parameters::parameters_ui(app, ui);
        graph::plot_ui(app, ui);
        ui.horizontal(|ui: &mut Ui| {
            general::play_ui(app, ui);
            general::info_ui(app, ui);
        });
    });
}

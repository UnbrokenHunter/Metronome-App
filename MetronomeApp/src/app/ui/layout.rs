use crate::app::ui::graph::graph_layout;
use crate::app::ui::logs::logs_panel_layout;
use crate::app::ui::parameters::parameters_layout;
use crate::app::{AppData, Menus};
use eframe::egui::{self, Context, Ui};

pub fn layout(app: &mut AppData, ctx: &Context) {
    header_ui(app, ctx);
    left_panel_ui(app, ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        main_panel_ui(app, ui);
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

fn left_panel_ui(app: &mut AppData, ctx: &Context) {
    if app.runtime.menu == Menus::Metronome {
        parameters_layout(app, ctx);
    } else if app.runtime.menu == Menus::Logs {
        logs_panel_layout(app, ctx);
    }
}

fn main_panel_ui(app: &mut AppData, ui: &mut Ui) {
    if app.runtime.menu == Menus::Metronome {
        graph_layout(app, ui);
    } else if app.runtime.menu == Menus::Logs {
    }
}

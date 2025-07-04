use crate::app::ui::accents::{accents_layout, accents_panel_layout};
use crate::app::ui::graph::graph_layout;
use crate::app::ui::logs::{logs_center_layout, logs_panel_layout};
use crate::app::ui::parameters::parameters_layout;
use crate::app::ui::settings::{settings_center_layout, settings_panel_layout};
use crate::app::ui::tabs::tabs_layout;
use crate::app::{AppData, Menus, logic};
use eframe::egui::{self, Context, Ui};

pub fn layout(app: &mut AppData, ctx: &Context) {
    header_panel_ui(app, ctx);
    left_panel_ui(app, ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        main_panel_ui(app, ui);
    });
}

fn header_panel_ui(app: &mut AppData, ctx: &Context) {
    tabs_layout(app, ctx);
}

fn left_panel_ui(app: &mut AppData, ctx: &Context) {
    if app.runtime.menu == Menus::Metronome {
        parameters_layout(app, ctx);
    } else if app.runtime.menu == Menus::Accents {
        accents_panel_layout(app, ctx);
    } else if app.runtime.menu == Menus::Logs {
        logs_panel_layout(app, ctx);
    } else if app.runtime.menu == Menus::Settings {
        settings_panel_layout(app, ctx);
    }
}

fn main_panel_ui(app: &mut AppData, ui: &mut Ui) {
    logic::tempo::calculate_tempo(app);

    if app.runtime.menu == Menus::Metronome {
        graph_layout(app, ui);
    } else if app.runtime.menu == Menus::Accents {
        accents_layout(app, ui);
    } else if app.runtime.menu == Menus::Logs {
        logs_center_layout(app, ui);
    } else if app.runtime.menu == Menus::Settings {
        settings_center_layout(app, ui)
    }
}

use crate::app::ui::accents::{accents_panel, accents_side};
use crate::app::ui::logs::{logs_panel, logs_side};
use crate::app::ui::settings::{settings_center_layout, settings_panel_layout};
use crate::app::ui::tabs::tabs_layout;
use crate::app::{logic, AppData, Menus};
use eframe::egui::{self, Context, Ui};
use crate::app::ui::metronome::{metronome_panel, metronome_side};

pub fn layout(app: &mut AppData, ctx: &Context) {
    header_panel_ui(app, ctx);

    egui::SidePanel::left("accents_panel")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                left_panel_ui(app, ui);
            });
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        main_panel_ui(app, ui);
    });
}

fn header_panel_ui(app: &mut AppData, ctx: &Context) {
    tabs_layout(app, ctx);
}

fn left_panel_ui(app: &mut AppData, ui: &mut Ui) {
    if app.runtime.menu == Menus::Metronome {
        metronome_side(app, ui);
    } else if app.runtime.menu == Menus::Accents {
        accents_side(app, ui);
    } else if app.runtime.menu == Menus::Logs {
        logs_side(app, ui);
    } else if app.runtime.menu == Menus::Settings {
        settings_panel_layout(app, ui);
    }
}

fn main_panel_ui(app: &mut AppData, ui: &mut Ui) {
    logic::tempo::calculate_tempo(app);

    if app.runtime.menu == Menus::Metronome {
        metronome_panel(app, ui);
    } else if app.runtime.menu == Menus::Accents {
        accents_panel(app, ui);
    } else if app.runtime.menu == Menus::Logs {
        logs_panel(app, ui);
    } else if app.runtime.menu == Menus::Settings {
        settings_center_layout(app, ui)
    }
}

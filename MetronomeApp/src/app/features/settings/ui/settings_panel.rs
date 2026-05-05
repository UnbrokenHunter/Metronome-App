use crate::app::AppData;
use eframe::egui::Ui;

use super::panel::{settings_general, settings_logs};

pub fn settings_panel(app: &mut AppData, ui: &mut Ui) {
    match app.runtime.menu_state {
        0 => settings_general(app, ui),
        1 => settings_logs(app, ui),
        _ => settings_general(app, ui),
    }
}

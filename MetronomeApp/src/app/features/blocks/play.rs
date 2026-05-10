use eframe::egui::{self, Ui};
use crate::app::features::logs::logic::logs;
use crate::app::AppData;
use crate::app::logic::popup_utils;

pub fn play(app: &mut AppData, ui: &mut Ui) {
    let size = [ui.available_width(), 30.0];

    if ui.add_sized(size, egui::Button::new("Play")).clicked() {
        app.runtime.playing = !app.runtime.playing;
    }
}

pub fn reset(app: &mut AppData, ui: &mut Ui) {
    let size = [ui.available_width(), 30.0];

    if ui.add_sized(size, egui::Button::new("Reset")).clicked() {
        if app.settings.do_title_popup {
            app.runtime.pending_log_title.clear();
            app.runtime.show_log_title_popup = true;
        } else {
            logs::try_add_log(app, None);
            app.reset_metronome();
        }
    }

    if let Some(title) = popup_utils::centered_text_input_popup(
        ui.ctx(),
        &mut app.runtime.show_log_title_popup,
        "Log Title",
        "Description",
        &mut app.runtime.pending_log_title,
        "Save",
    ) {
        logs::try_add_log(app, Some(title));
        app.reset_metronome();
        app.runtime.pending_log_title.clear();
    }
}

pub fn revert_defaults(app: &mut AppData, ui: &mut Ui) {
    let size = [ui.available_width(), 30.0];

    if ui.add_sized(size, egui::Button::new("Revert Defaults")).clicked() {
        if app.settings.do_title_popup {
            app.runtime.pending_log_title.clear();
            app.runtime.show_log_title_popup = true;
        } else {
            logs::try_add_log(app, None);
            app.reset_all_parameters();
        }
    }

    if let Some(title) = popup_utils::centered_text_input_popup(
        ui.ctx(),
        &mut app.runtime.show_log_title_popup,
        "Log Title",
        "Description",
        &mut app.runtime.pending_log_title,
        "Save",
    ) {
        logs::try_add_log(app, Some(title));
        app.reset_all_parameters();
        app.runtime.pending_log_title.clear();
    }
}
use crate::app::AppData;
use crate::app::data::PendingTitleAction;
use crate::app::features::logs::logic::logs;
use crate::app::logic::popup_utils;
use eframe::egui::{self, Ui};

pub fn play(app: &mut AppData, ui: &mut Ui) {
    let size = [ui.available_width(), 30.0];

    if ui.add_sized(size, egui::Button::new("Play")).clicked() {
        app.runtime.playing = !app.runtime.playing;
    }
}

pub fn reset(app: &mut AppData, ui: &mut Ui) {
    let size = [ui.available_width(), 30.0];

    if ui.add_sized(size, egui::Button::new("Reset")).clicked() {
        start_or_run(app, PendingTitleAction::Reset);
    }

    draw_title_popup_for(app, ui, PendingTitleAction::Reset);
}

pub fn revert_defaults(app: &mut AppData, ui: &mut Ui) {
    let size = [ui.available_width(), 30.0];

    if ui
        .add_sized(size, egui::Button::new("Revert Defaults"))
        .clicked()
    {
        start_or_run(app, PendingTitleAction::RevertDefaults);
    }

    draw_title_popup_for(app, ui, PendingTitleAction::RevertDefaults);
}

fn draw_title_popup_for(app: &mut AppData, ui: &mut Ui, action: PendingTitleAction) {
    if app.runtime.pending_title_action != Some(action) {
        return;
    }

    match popup_utils::centered_text_input_popup(
        ui.ctx(),
        true,
        "Log Title",
        "Description",
        &mut app.runtime.pending_log_title,
        "Save",
    ) {
        Some(Some(title)) => finish_action(app, action, Some(title)),
        Some(None) => {
            app.runtime.pending_title_action = None;
            app.runtime.pending_log_title.clear();
        }
        None => {}
    }
}

fn start_or_run(app: &mut AppData, action: PendingTitleAction) {
    if app.settings.do_title_popup {
        app.runtime.pending_log_title.clear();
        app.runtime.pending_title_action = Some(action);
    } else {
        finish_action(app, action, None);
    }
}

fn finish_action(app: &mut AppData, action: PendingTitleAction, title: Option<String>) {
    logs::try_add_log(app, title);

    match action {
        PendingTitleAction::Reset => app.reset_metronome(),
        PendingTitleAction::RevertDefaults => app.reset_all_parameters(),
    }

    app.runtime.pending_log_title.clear();
    app.runtime.pending_title_action = None;
}

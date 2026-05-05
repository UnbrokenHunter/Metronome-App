use crate::app::features::graph::plot::draw_plot;
use crate::app::{
    logic::clock::{format_date, format_time, weekday_from_unix_ms},
    AppData,
};
use eframe::egui::{self, RichText, ScrollArea, TextEdit, TextStyle, Ui};

pub fn logs_panel(app: &mut AppData, ui: &mut Ui) {
    let mut to_delete = None;
    let selected_index = app.runtime.menu_state as usize;

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            let Some(log) = app.practice.logs.get_mut(selected_index) else {
                ui.label("No log selected.");
                return;
            };

            egui::Frame::group(ui.style()).show(ui, |ui| {
                log_header(ui, log);
                ui.separator();

                log_title_editor(ui, log);
                ui.separator();

                log_main_content(ui, log);
                ui.separator();

                log_notes_editor(ui, log);
                ui.separator();

                if full_width_button(ui, "Copy To Clipboard") {
                    ui.ctx().copy_text(log_text_info(log));
                }

                if full_width_button(ui, "Delete Log") {
                    to_delete = Some(selected_index);
                }
            });
        });
    });

    if let Some(index) = to_delete {
        delete_log(app, index);
    }
}

fn log_header(ui: &mut Ui, log: &crate::app::types::PracticeLog) {
    let color = ui
        .visuals()
        .override_text_color
        .unwrap_or(ui.visuals().text_color());
    ui.label(
        RichText::new(format!(
            "{}, {}",
            weekday_from_unix_ms(log.time_started),
            format_date(log.time_started, None)
        ))
        .size(28.0)
        .color(color)
        .strong(),
    );
}

fn log_title_editor(ui: &mut Ui, log: &mut crate::app::types::PracticeLog) {
    ui.add_sized(
        [ui.available_width(), 27.0],
        TextEdit::singleline(&mut log.title)
            .font(TextStyle::Heading)
            .hint_text("Title..."),
    );
}

fn log_main_content(ui: &mut Ui, log: &crate::app::types::PracticeLog) {
    ui.horizontal(|ui| {
        let total_width = ui.available_width();
        let plot_width = total_width * 2.0 / 3.0;
        let info_width = total_width - plot_width;

        ui.allocate_ui_with_layout(
            egui::vec2(plot_width, ui.available_height()),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                draw_plot(ui, &log.points);
            },
        );

        ui.allocate_ui_with_layout(
            egui::vec2(info_width, ui.available_height()),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                log_info_panel(ui, log);
            },
        );
    });
}

fn log_info_panel(ui: &mut Ui, log: &crate::app::types::PracticeLog) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.label(RichText::new("Info:").size(20.0).strong());
        ui.separator();

        info_section(ui, "General", |ui| {
            ui.label(format!(
                "Duration: {}",
                format_time(log.duration_ms as u128)
            ));
        });

        ui.separator();

        info_section(ui, "Tempo", |ui| {
            ui.label(format!("Starting Tempo: {}", log.min_tempo));
            ui.label(format!("Ending Tempo: {}", log.max_tempo));
        });

        ui.separator();

        info_section(ui, "Statistics", |ui| {
            ui.label(format!("Average Tempo: {}", log.average_tempo));
            ui.label(format!("Average Delta: {}", log.average_delta));
        });
    });
}

fn info_section(ui: &mut Ui, title: &str, contents: impl FnOnce(&mut Ui)) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.heading(title);
        ui.separator();

        contents(ui);
    });
}

fn log_notes_editor(ui: &mut Ui, log: &mut crate::app::types::PracticeLog) {
    ui.add_sized(
        [ui.available_width(), 100.0],
        TextEdit::multiline(&mut log.notes).hint_text("Add Notes..."),
    );
}

fn full_width_button(ui: &mut Ui, text: &str) -> bool {
    ui.add_sized([ui.available_width(), 30.0], egui::Button::new(text))
        .clicked()
}

fn delete_log(app: &mut AppData, index: usize) {
    if index < app.practice.logs.len() {
        app.practice.logs.remove(index);

        app.runtime.menu_state = app.runtime.menu_state.saturating_sub(1);
    }
}

fn log_text_info(log: &crate::app::types::PracticeLog) -> String {
    let date = format_date(log.time_started, None);

    let title_line = if log.title.is_empty() {
        date
    } else {
        format!("{}, {date}", log.title)
    };

    format!(
        "\
{title_line}
------------------------------------
Duration:        {}
Average Tempo:   {}
Average Delta:   {}
Starting Tempo:  {}
Ending Tempo:    {}
Notes:
{}

",
        format_time(log.duration_ms as u128),
        log.average_tempo,
        log.average_delta,
        log.min_tempo,
        log.max_tempo,
        log.notes.trim()
    )
}

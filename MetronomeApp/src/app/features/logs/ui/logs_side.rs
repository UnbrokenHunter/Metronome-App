use std::collections::BTreeMap;

use crate::app::{AppData, logic, types::PracticeLog};
use eframe::egui::{self, ScrollArea, Ui};

pub fn logs_side(app: &mut AppData, ui: &mut Ui) {
    ui.allocate_ui_with_layout(
        egui::vec2(200.0, ui.available_height()),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                logs_side_contents(app, ui);
            });
        },
    );
}

fn logs_side_contents(app: &mut AppData, ui: &mut Ui) {
    let grouped = grouped_logs(&app.practice.logs);

    for (day, logs) in grouped {
        let Some((_, first_log)) = logs.first() else {
            continue;
        };

        let date_label = logic::clock::format_date(first_log.time_started, None);

        egui::CollapsingHeader::new(date_label)
            .default_open(false)
            .id_salt(format!("logs_day_{day}"))
            .show(ui, |ui| {
                for (index, log) in logs {
                    if log_card(ui, index, log) {
                        app.runtime.menu_state = index as u32;
                    }

                    ui.add_space(8.0);
                }
            });
    }
}

fn grouped_logs(logs: &[PracticeLog]) -> BTreeMap<u64, Vec<(usize, &PracticeLog)>> {
    let mut grouped: BTreeMap<u64, Vec<(usize, &PracticeLog)>> = BTreeMap::new();

    for (index, log) in logs.iter().enumerate() {
        let day = logic::clock::days_since_epoch(log.time_started);
        grouped.entry(day).or_default().push((index, log));
    }

    grouped
}

fn log_card(ui: &mut Ui, index: usize, log: &PracticeLog) -> bool {
    let response = egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.heading(log_card_title(log));
        ui.separator();

        ui.label(format!(
            "Duration: {}",
            logic::clock::format_time(log.duration_ms as u128)
        ));

        if log.notes.is_empty() {
            ui.label(format!("Average Tempo: {}", log.average_tempo));
            ui.label(format!("Average Delta: {}", log.average_delta));
        } else {
            ui.label(format!("Notes: {}", log.notes));
        }
    });

    ui.interact(
        response.response.rect,
        egui::Id::new(("log_card", index)),
        egui::Sense::click(),
    )
    .clicked()
}

fn log_card_title(log: &PracticeLog) -> String {
    if log.title.is_empty() {
        logic::clock::format_date(log.time_started, None)
    } else {
        log.title.clone()
    }
}

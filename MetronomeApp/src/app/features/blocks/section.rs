use eframe::egui::{Frame, Ui};

pub(super) fn section(
    ui: &mut Ui,
    title: &str,
    contents: impl FnOnce(&mut Ui),
) {
    Frame::group(ui.style()).show(ui, |ui| {
        ui.label(title);
        ui.separator();
        contents(ui);
    });
}
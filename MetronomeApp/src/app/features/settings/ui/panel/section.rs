use eframe::egui::{self, Ui};

pub(super) fn settings_section(
    ui: &mut Ui,
    title: &str,
    description: &str,
    contents: impl FnOnce(&mut Ui),
) {
    ui.heading(title);

    contents(ui);

    ui.label(
        egui::RichText::new(description)
            .small()
            .color(ui.visuals().weak_text_color()),
    );

    ui.separator();
    ui.add_space(8.0);
}

pub(super) fn full_width_button(ui: &mut Ui, text: &str) -> bool {
    ui.add_sized([ui.available_width(), 30.0], egui::Button::new(text))
        .clicked()
}

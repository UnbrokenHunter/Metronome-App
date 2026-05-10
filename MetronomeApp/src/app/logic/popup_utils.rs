#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]

use egui::{Align2, Button, Context, Frame, Response, RichText, Ui, Vec2};

/// Button that opens a styled popup menu.
/// Returns the button response.
pub fn popup_menu_button(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    button_text: impl Into<egui::WidgetText>,
    content: impl FnOnce(&mut Ui),
) -> Response {
    let popup_id = ui.make_persistent_id(id);

    let btn = ui.button(button_text);

    if btn.clicked() {
        ui.ctx().memory_mut(|m| m.toggle_popup(popup_id));
    }

    egui::popup::popup_below_widget(
        ui,
        popup_id,
        &btn,
        egui::popup::PopupCloseBehavior::CloseOnClickOutside,
        |ui| {
            Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(160.0);
                content(ui);
            });
        },
    );

    btn
}

/// A single clickable menu item.
/// Returns `true` when clicked.
pub fn popup_menu_item(ui: &mut Ui, label: impl Into<egui::WidgetText>) -> bool {
    ui.add_sized(
        [ui.available_width(), 24.0],
        Button::new(label).frame(false),
    )
    .clicked()
}

/// A disabled-looking menu item.
pub fn popup_menu_disabled_item(ui: &mut Ui, label: impl Into<String>) {
    ui.add_enabled(
        false,
        Button::new(RichText::new(label.into()).weak()).frame(false),
    );
}

/// Small gray heading inside a popup menu.
pub fn popup_menu_heading(ui: &mut Ui, label: &str) {
    ui.label(RichText::new(label).weak().size(13.0));
    ui.separator();
}

/// Popup menu that chooses one value from a list.
/// Returns `Some(value)` when the user picks something.
pub fn popup_select_menu<T, L>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    button_text: impl Into<egui::WidgetText>,
    options: &[T],
    mut to_label: L,
) -> Option<T>
where
    T: Clone,
    L: FnMut(&T) -> String,
{
    let mut picked = None;
    let popup_id = ui.make_persistent_id(id);

    let btn = ui.button(button_text);

    if btn.clicked() {
        ui.ctx().memory_mut(|m| m.toggle_popup(popup_id));
    }

    egui::popup::popup_below_widget(
        ui,
        popup_id,
        &btn,
        egui::popup::PopupCloseBehavior::CloseOnClickOutside,
        |ui| {
            Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(160.0);

                for option in options {
                    if popup_menu_item(ui, to_label(option)) {
                        picked = Some(option.clone());
                        ui.ctx().memory_mut(|m| m.close_popup());
                    }
                }
            });
        },
    );

    picked
}

/// Popup menu for editing some external state.
/// Returns `true` if the popup content reports a change.
pub fn editable_popup_menu(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    button_text: impl Into<egui::WidgetText>,
    content: impl FnOnce(&mut Ui) -> bool,
) -> bool {
    let mut changed = false;
    let popup_id = ui.make_persistent_id(id);

    let btn = ui.button(button_text);

    if btn.clicked() {
        ui.ctx().memory_mut(|m| m.toggle_popup(popup_id));
    }

    egui::popup::popup_below_widget(
        ui,
        popup_id,
        &btn,
        egui::popup::PopupCloseBehavior::CloseOnClickOutside,
        |ui| {
            Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(180.0);
                changed = content(ui);
            });
        },
    );

    changed
}

/// Confirmation popup opened by a button.
/// Returns `true` only when the confirm button is clicked.
pub fn confirm_popup_button(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    button_text: impl Into<egui::WidgetText>,
    message: &str,
    confirm_text: &str,
) -> bool {
    let popup_id = ui.make_persistent_id(id);
    let mut confirmed = false;

    let btn = ui.button(button_text);

    if btn.clicked() {
        ui.ctx().memory_mut(|m| m.toggle_popup(popup_id));
    }

    egui::popup::popup_below_widget(
        ui,
        popup_id,
        &btn,
        egui::popup::PopupCloseBehavior::CloseOnClickOutside,
        |ui| {
            Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(220.0);

                ui.label(message);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        ui.ctx().memory_mut(|m| m.close_popup());
                    }

                    let confirm =
                        Button::new(RichText::new(confirm_text).color(ui.visuals().warn_fg_color));

                    if ui.add(confirm).clicked() {
                        confirmed = true;
                        ui.ctx().memory_mut(|m| m.close_popup());
                    }
                });
            });
        },
    );

    confirmed
}

pub fn centered_text_input_popup(
    ctx: &egui::Context,
    is_open: bool,
    title: &str,
    label: &str,
    text: &mut String,
    confirm_text: &str,
) -> Option<Option<String>> {
    if !is_open {
        return None;
    }

    let mut result = None;
    let input_id = egui::Id::new((title, "text_input"));

    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(260.0);
            ui.label(label);

            let text_response = ui.add(
                egui::TextEdit::singleline(text)
                    .id(input_id)
                    .desired_width(ui.available_width()),
            );

            text_response.request_focus();

            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    result = Some(None);
                }

                if ui.button(confirm_text).clicked() || enter_pressed {
                    result = Some(Some(text.trim().to_string()));
                }
            });
        });

    result
}

pub fn confirmation_popup(
    ctx: &Context,
    is_open: bool,
    title: &str,
    message: &str,
    confirm_text: &str,
) -> Option<bool> {
    if !is_open {
        return None;
    }

    let mut result = None;

    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.label(message);
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    result = Some(false);
                }

                let confirm_button =
                    Button::new(RichText::new(confirm_text).color(ui.visuals().warn_fg_color));

                if ui.add(confirm_button).clicked() {
                    result = Some(true);
                }
            });
        });

    result
}

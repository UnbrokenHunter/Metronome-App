#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]

use egui::{Button, Color32, ComboBox, Frame, Id, Response, RichText, ScrollArea, Ui};
use std::fmt::Display;

pub const SPACING: f32 = 2.0;

/// Group with a title and separator.
pub fn section_group(ui: &mut Ui, title: &str, content: impl FnOnce(&mut Ui)) {
    ui.vertical(|ui| {
        Frame::group(ui.style()).show(ui, |ui| {
            ui.label(RichText::new(title).color(Color32::GRAY).size(15.0));
            ui.separator();
            content(ui);
            ui.add_space(SPACING);
        });
    });
}

/// Button that toggles a popup; returns the button `Response`.
pub fn popup_button(
    ui: &mut Ui,
    popup_id: Id,
    button_text: impl Into<egui::WidgetText>, // <- change this
    content: impl FnOnce(&mut Ui),
) -> egui::Response {
    let btn = ui.button(button_text);
    if btn.clicked() {
        ui.ctx().memory_mut(|m| m.toggle_popup(popup_id));
    }
    egui::popup::popup_below_widget(
        ui,
        popup_id,
        &btn,
        egui::popup::PopupCloseBehavior::CloseOnClickOutside,
        content,
    );
    btn
}

/// Simple multiselect grid. Returns `true` if `selected` changed.
pub fn multiselect_grid<T, L>(
    ui: &mut Ui,
    id: Id,
    all_options: &[T],
    selected: &mut Vec<T>,
    columns: usize,
    mut to_label: L,
) -> bool
where
    T: Clone + PartialEq,
    L: FnMut(&T) -> String,
{
    let mut changed = false;

    egui::Grid::new(id)
        .num_columns(columns.max(1))
        .spacing([12.0, 4.0])
        .show(ui, |ui| {
            for (i, item) in all_options.iter().enumerate() {
                let mut is_selected = selected.contains(item);
                let resp = ui.checkbox(&mut is_selected, to_label(item));
                if resp.changed() {
                    changed = true;
                    if is_selected {
                        if !selected.contains(item) {
                            selected.push(item.clone());
                        }
                    } else {
                        // allow deselect last; change to `if selected.len()>1` if you want to keep one
                        selected.retain(|x| x != item);
                    }
                }
                if (i + 1) % columns == 0 {
                    ui.end_row();
                }
            }
        });

    changed
}

/// Preset picker (combo) that writes a `(name, value)` pair into `selected`.
pub fn draw_presets<T: Clone>(
    ui: &mut Ui,
    label: &str,
    presets: &[(String, T)],
    selected: &mut (String, T),
) -> bool {
    let mut changed = false;

    ui.label(label);

    let current_idx = presets
        .iter()
        .position(|(n, _)| n == &selected.0)
        .unwrap_or(0);

    let mut idx = current_idx;
    ComboBox::from_id_salt(Id::new((label, "presets")))
        .selected_text(&presets[current_idx].0)
        .show_ui(ui, |ui| {
            for (i, (name, _)) in presets.iter().enumerate() {
                if ui.selectable_label(idx == i, name).clicked() {
                    idx = i;
                }
            }
        });

    if idx != current_idx {
        *selected = presets[idx].clone();
        changed = true;
    }

    changed
}

/// Presets + “custom” flow. `custom_name` is the label for the last preset option.
pub fn draw_presets_with_multiselect_custom<T, L>(
    ui: &mut Ui,
    label: &str,
    presets: &[(String, Vec<T>)],
    selected: &mut (String, Vec<T>),
    all_items: &[T],
    columns: usize,
    to_label: L,
    custom_name: &str,
) where
    T: Clone + PartialEq,
    L: FnMut(&T) -> String + Copy,
{
    let mut tmp = selected.clone();
    let _ = draw_presets(ui, label, presets, &mut tmp);

    let custom_idx = presets
        .iter()
        .position(|(n, _)| n == custom_name)
        .unwrap_or(presets.len().saturating_sub(1));

    let chosen_idx = presets.iter().position(|(n, _)| n == &tmp.0).unwrap_or(0);

    if chosen_idx != custom_idx {
        *selected = presets[chosen_idx].clone();
        return;
    } else {
        selected.0 = presets[custom_idx].0.clone();
    }

    let popup_id = Id::new((label, "custom_multiselect_popup"));
    popup_button(ui, popup_id, format!("Edit {label}"), |ui| {
        let _ = multiselect_grid(
            ui,
            Id::new((label, "grid")),
            all_items,
            &mut selected.1,
            columns,
            to_label,
        );
    });
}

/// Draw & edit a `Vec<T>` with pluggable editor. Returns `true` if items changed.
pub fn editable_array<T, F, G>(
    ui: &mut Ui,
    items: &mut Vec<T>,
    new_item_state: &mut T,
    mut editor: F,
    mut make_default: G,
    add_mode: AddMode,
) -> bool
where
    T: PartialEq + Clone,
    F: FnMut(&mut Ui, &mut T) -> Response,
    G: FnMut() -> T,
{
    let mut changed = false;
    let mut to_remove: Option<usize> = None;

    Frame::group(ui.style()).show(ui, |ui| {
        for (i, item) in items.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                let resp = editor(ui, item);
                if resp.changed() {
                    changed = true;
                }
                ui.separator();
                let remove = ui
                    .add(Button::new("🗑").min_size(egui::vec2(24.0, 0.0)))
                    .on_hover_text("Remove this element");
                if remove.clicked() {
                    to_remove = Some(i);
                }
            });
            ui.add_space(4.0);
        }

        if let Some(i) = to_remove {
            items.remove(i);
            changed = true;
        }

        ui.horizontal(|ui| {
            let resp = editor(ui, new_item_state);
            match add_mode {
                AddMode::OnChange => {
                    if resp.changed() {
                        items.push(new_item_state.clone());
                        *new_item_state = make_default();
                        changed = true;
                    }
                }
                AddMode::OnButton(label) => {
                    ui.separator();
                    if ui.button(label).clicked() {
                        items.push(new_item_state.clone());
                        *new_item_state = make_default();
                        changed = true;
                    }
                }
            }
        });
    });

    changed
}

pub fn text_apply_on_blur<T, S>(
    ui: &mut egui::Ui,
    id: egui::Id,
    text: &mut String,
    mut editor: T,
    mut should_apply: Option<S>,
) -> bool
where
    T: FnMut(&mut egui::Ui, &mut String) -> egui::Response,
    S: for<'a, 'b> FnMut(&'a str, &'b str) -> bool,
{
    let mut buf = ui
        .ctx()
        .data_mut(|d| d.get_persisted::<String>(id))
        .unwrap_or_else(|| text.clone());

    let resp = editor(ui, &mut buf);

    if resp.has_focus() || resp.changed() {
        ui.ctx().data_mut(|d| d.insert_persisted(id, buf.clone()));
    }

    let mut committed = false;
    if resp.lost_focus() && !resp.has_focus() && buf.as_str() != text.as_str() {
        let ok = should_apply
            .as_mut()
            .is_none_or(|p| p(text.as_str(), buf.as_str()));
        if ok {
            *text = buf.clone();
            committed = true;
        }
        ui.ctx().data_mut(|d| d.remove::<String>(id));
    }
    committed
}

/// Horizontal tabs (scrollable if overflow). `tabs` is a slice; `T: Eq + Display + Clone`.
pub fn tabs<T: Eq + Display + Clone>(ui: &mut Ui, selected: &mut T, tabs: &[T]) -> Option<usize> {
    let tab_w = (ui.available_width() / 10.0).max(60.0);
    let tab_h = 25.0;
    let mut clicked_idx = None;

    ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            for (i, tab) in tabs.iter().cloned().enumerate() {
                let is_selected = *selected == tab;
                let mut btn = Button::new(format!("{tab}"));
                if is_selected {
                    btn = btn.fill(ui.visuals().selection.bg_fill);
                }
                if ui.add_sized([tab_w, tab_h], btn).clicked() {
                    *selected = tab;
                    clicked_idx = Some(i);
                }
            }
        });
    });

    clicked_idx
}

/// When to commit items in `editable_array`.
pub enum AddMode {
    OnChange,
    OnButton(&'static str),
}

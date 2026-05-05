use std::fmt;
use std::fmt::Display;

use egui::{Button, Context, ScrollArea, Ui};
use serde::{Deserialize, Serialize};

use crate::app::features::registry::Registry;
use crate::app::AppData;

pub fn draw_layout(
    app: &mut AppData,
    selected_tab: &mut Menu,
    registry: &mut Registry,
    ctx: &Context,
) {
    egui::TopBottomPanel::top("tabs")
        .resizable(false)
        .show(ctx, |ui| {
            tabs(ui, selected_tab, &registry.get_all_keys());
        });

    egui::SidePanel::left("sidebar")
        .resizable(false)
        .show(ctx, |ui| {
            registry.get_mut(*selected_tab).draw_sidebar(app, ui);
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        registry.get_mut(*selected_tab).draw_panel(app, ui);
    });
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Menu {
    Home,
    Accents,
    Logs,
    Settings,
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Menu::Home => "Home",
            Menu::Accents => "Accents",
            Menu::Logs => "Logs",
            Menu::Settings => "Settings",
        })
    }
}

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
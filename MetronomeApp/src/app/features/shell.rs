use egui::Context;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::app::features::registry::Registry;
use crate::app::logic::ui_utils::tabs;
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

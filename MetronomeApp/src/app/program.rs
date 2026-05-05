use eframe::egui::Context;
use eframe::Frame;

use crate::app::logic::logs;
use crate::app::types::AppData;

use super::features::{shell, Menu, Registry};
use super::logic::{clock, keyboard, metronome};

pub struct Window {
    data: AppData,
    registry: Registry,
    selected_menu: Menu,
    started: bool,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            selected_menu: Menu::Home,
            data: AppData::default(),
            registry: Registry::new(),
            started: false,
        }
    }
}

impl Window {
    fn startup(&mut self, ctx: &egui::Context) {
        // Put all your "Start" logic here
        println!("Window has started!");
        // theme(ctx).apply_to_ctx(ctx);
        self.data.settings.color_scheme.apply_to_ctx(ctx);
        // keyboard_state::init_keyboard_ctx(ctx);
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if !self.started {
            self.started = true;
            self.startup(ctx); // Call once at the beginning
        }

        // tabs_layout::tabs_layout(&mut self.data, ctx);
        // layout::layout(&mut self.data, ctx);

        keyboard::check_keyboard(&mut self.data, ctx.clone());
        clock::update_time(&mut self.data.runtime.time_data, self.data.runtime.playing);
        metronome::update_metronome(&mut self.data);

        shell::draw_layout(
            &mut self.data,
            &mut self.selected_menu,
            &mut self.registry,
            ctx,
        );

        ctx.request_repaint();
    }

}

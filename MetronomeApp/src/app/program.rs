use eframe::Frame;
use eframe::egui::Context;

use crate::app::AppData;

use super::features::{Menu, Registry, shell};
use super::logic::{clock, keyboard, metronome, updates};

pub struct Window {
    data: AppData,
    registry: Registry,
    selected_menu: Menu,
    updates: updates::UpdateRuntime,
    started: bool,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            selected_menu: Menu::Home,
            data: AppData::default(),
            registry: Registry::new(),
            updates: updates::UpdateRuntime::default(),
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

        updates::start_update_check(&mut self.updates);
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if !self.started {
            self.started = true;
            self.startup(ctx);
        }

        updates::receive_update_messages(&mut self.updates);

        keyboard::check_keyboard(&mut self.data, ctx.clone());
        clock::update_time(&mut self.data.runtime.time_data, self.data.runtime.playing);
        metronome::update_metronome(&mut self.data);

        shell::draw_layout(
            &mut self.data,
            &mut self.selected_menu,
            &mut self.registry,
            ctx,
        );

        updates::draw_update_popup(ctx, &mut self.updates);

        ctx.request_repaint();
    }
}

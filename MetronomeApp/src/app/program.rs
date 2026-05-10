use eframe::Frame;
use eframe::egui::Context;

use super::features::{Menu, Registry, calculate_tempo, shell, update_metronome};
use crate::app::AppData;
use crate::app::systems::time::clock;
use crate::app::systems::{audio, deployment, peripherals};

pub struct Window {
    data: AppData,
    registry: Registry,
    selected_menu: Menu,
    updates: deployment::UpdateRuntime,
    started: bool,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            selected_menu: Menu::Home,
            data: AppData::default(),
            registry: Registry::new(),
            updates: deployment::UpdateRuntime::default(),
            started: false,
        }
    }
}

impl Window {
    fn startup(&mut self, ctx: &egui::Context) {
        println!("Window has started!");

        self.data.current_theme().apply_to_ctx(ctx);
        peripherals::init_keyboard_ctx(ctx);

        deployment::start_update_check(&mut self.updates);
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if !self.started {
            self.started = true;
            self.startup(ctx);
        }

        deployment::receive_update_messages(&mut self.updates);

        audio::cleanup(); // Get rid of old sounds

        peripherals::check_keyboard(&mut self.data);
        clock::update_time(&mut self.data.runtime.time_data, self.data.runtime.playing);
        update_metronome(&mut self.data);
        calculate_tempo(&mut self.data);

        shell::draw_layout(
            &mut self.data,
            &mut self.selected_menu,
            &mut self.registry,
            ctx,
        );

        deployment::draw_update_popup(ctx, &mut self.updates);

        ctx.request_repaint();
    }
}

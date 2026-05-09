use eframe::egui::Context;
use eframe::Frame;

use super::features::{shell, Menu, Registry};
use super::logic::{clock, metronome, tempo};
use crate::app::systems::{audio, deployment, peripherals};
use crate::app::AppData;

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
        crate::app::systems::colors::themes::theme(
            &self.data.themes,
            self.data.settings.selected_theme_index,
        )
            .apply_to_ctx(ctx);
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
        metronome::update_metronome(&mut self.data);
        tempo::calculate_tempo(&mut self.data);

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

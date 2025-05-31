use crate::app::ui::settings;
use crate::app::{MyApp, logic};
use eframe::egui::{self, Context, Ui};

pub fn layout(app: &mut MyApp, ctx: &Context) {
    settings_ui(app, ctx);

    if app.playing {
        egui::SidePanel::right("info")
            .resizable(false)
            .show(ctx, |ui| {
                settings::info_ui(app, ui);
                settings::play_ui(app, ui);
            });
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        main_ui(app, ui);
    });
}

fn settings_ui(app: &mut MyApp, ctx: &Context) {
    // IF hovering over the thing, or the thing is displayed and you are hovering over that
    let mut hover = false;
    let tab_size = 25.0;
    if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
        hover = pos.x < tab_size;
    } else {
        if hover {
            // hover = pos.x < tab_size;
        }
    }

    // Paused
    if !app.playing || hover {
        egui::SidePanel::left("settings")
            .resizable(false)
            .show(ctx, |ui| {
                settings::practice_ui(app, ui);
                settings::tempo_ui(app, ui);

                settings::growth_ui(app, ui);
                settings::sound_ui(app, ui);
            });
    } else {
        // Unpaused
        egui::SidePanel::left("settings")
            .resizable(false)
            .frame(egui::Frame {
                fill: ctx.style().visuals.window_fill(), // Matches background
                inner_margin: egui::Margin::same(0),
                outer_margin: egui::Margin::same(0),
                ..Default::default()
            })
            .max_width(tab_size)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../../../assets/images/tab.png"))
                            .fit_to_exact_size(egui::Vec2 {
                                x: (tab_size),
                                y: (tab_size),
                            })
                            .maintain_aspect_ratio(true),
                    );
                });
            });
    }
}

fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        logic::tempo::calculate_tempo(app);

        settings::plot_ui(app, ui);
        if !app.playing {
            ui.horizontal(|ui: &mut Ui| {
                settings::play_ui(app, ui);
                settings::info_ui(app, ui);
            });
        }
    });
}

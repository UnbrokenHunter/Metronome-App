use crate::app::{
    logic::accents::get_accent_and_beat_index,
    types::{AccentData, BeatData, BeatState},
    AppData,
};
use eframe::egui::{
    self, Align, Color32, ComboBox, CornerRadius, Frame, ImageButton, Layout, Rect, ScrollArea,
    Sense, Stroke, TextEdit, TextStyle, Ui, Vec2,
};

const SMALL_ICON_SIZE: f32 = 20.0;
const TINY_ICON_SIZE: f32 = 10.0;
const SAVE_ICON_SIZE: f32 = 25.0;
const BEAT_BUTTON_WIDTH: f32 = 20.0;
const BEAT_BUTTON_HEIGHT: f32 = 8.0;
const BEAT_MENU_STATE_MULTIPLIER: usize = 10_000;

#[derive(Clone, Copy)]
enum AccentAction {
    MoveUp(usize),
    MoveDown(usize),
    Duplicate(usize),
    Insert(usize),
    Delete(usize),
}

#[derive(Clone, Copy)]
struct BeatColors {
    override_color: Color32,
    downbeat: Color32,
    strong: Color32,
    weak: Color32,
    off: Color32,
}

pub fn accents_layout(app: &mut AppData, ui: &mut Ui) {
    Frame::group(ui.style()).show(ui, |ui| {
        draw_accent_top_bar(app, ui);

        ui.separator();

        Frame::group(ui.style()).show(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    let total_width = ui.available_width();
                    let mut accent_index = 0;

                    while accent_index < app.parameters.accents.accents.len() {
                        draw_accent(app, ui, accent_index, total_width);
                        accent_index += 1;
                    }
                });
            });
        });
    });
}

fn draw_accent_top_bar(app: &mut AppData, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if icon_button(
            ui,
            egui::include_image!("../../../../assets/icons/save.png"),
            SAVE_ICON_SIZE,
        ) {
            app.accent_presets
                .accent_chains
                .push(app.parameters.accents.clone());
        }

        let available_width = ui.available_width();

        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            let dropdown_width = 180.0;
            let header_width = available_width - dropdown_width;

            ui.add_sized(
                [header_width.max(0.0), 27.0],
                TextEdit::singleline(&mut app.parameters.accents.name)
                    .font(TextStyle::Heading)
                    .hint_text("Title..."),
            );

            let mut selected_preset_index = None;

            ComboBox::from_label("")
                .selected_text(truncate(&app.parameters.accents.name, 22))
                .height(300.0)
                .show_ui(ui, |ui| {
                    for (index, preset) in app.accent_presets.accent_chains.iter().enumerate() {
                        if ui
                            .selectable_label(
                                app.parameters.accents.name == preset.name,
                                &preset.name,
                            )
                            .clicked()
                        {
                            selected_preset_index = Some(index);
                        }
                    }
                });

            if let Some(index) = selected_preset_index {
                if let Some(preset) = app.accent_presets.accent_chains.get(index) {
                    app.parameters.accents = preset.clone();
                }
            }
        });
    });
}

fn draw_accent(app: &mut AppData, ui: &mut Ui, accent_index: usize, total_width: f32) {
    let current_click = get_accent_and_beat_index(app, app.runtime.last_click_accent as usize);
    let colors = beat_colors(app);
    let playing = app.runtime.playing;

    let mut action = None;

    let menu_state = &mut app.runtime.menu_state;
    let accent = app
        .parameters
        .accents
        .accents
        .get_mut(accent_index)
        .unwrap();

    ui.horizontal(|ui| {
        draw_accent_side_buttons(ui, accent_index, &mut action);

        ui.vertical(|ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                draw_accent_header(ui, accent, accent_index, total_width, &mut action);

                ui.separator();

                draw_accent_beats(
                    ui,
                    accent,
                    accent_index,
                    menu_state,
                    playing,
                    current_click,
                    colors,
                    &mut action,
                );
            });
        });
    });

    if let Some(action) = action {
        apply_accent_action(app, action);
    }
}

fn draw_accent_side_buttons(ui: &mut Ui, accent_index: usize, action: &mut Option<AccentAction>) {
    egui::Grid::new(format!("{accent_index}_insert_grid"))
        .num_columns(1)
        .striped(false)
        .show(ui, |ui| {
            if icon_button(
                ui,
                egui::include_image!("../../../../assets/icons/down_arrow.png"),
                SMALL_ICON_SIZE,
            ) {
                println!("First Insert clicked!");
                *action = Some(AccentAction::Insert(accent_index));
            }

            ui.end_row();

            if icon_button(
                ui,
                egui::include_image!("../../../../assets/icons/duplicate.png"),
                SMALL_ICON_SIZE,
            ) {
                println!("Duplicate Clicked!");
                *action = Some(AccentAction::Duplicate(accent_index));
            }

            ui.end_row();

            if icon_button(
                ui,
                egui::include_image!("../../../../assets/icons/down_arrow.png"),
                SMALL_ICON_SIZE,
            ) {
                println!("Second Insert clicked!");
                *action = Some(AccentAction::Insert(accent_index + 1));
            }

            ui.end_row();
        });
}

fn draw_accent_header(
    ui: &mut Ui,
    accent: &mut AccentData,
    accent_index: usize,
    total_width: f32,
    action: &mut Option<AccentAction>,
) {
    ui.horizontal(|ui| {
        if icon_button(
            ui,
            egui::include_image!("../../../../assets/icons/up.png"),
            SMALL_ICON_SIZE,
        ) {
            println!("Move Accent Up Clicked!");
            *action = Some(AccentAction::MoveUp(accent_index));
        }

        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.add_sized(
                [total_width / 2.5, 20.0],
                TextEdit::singleline(&mut accent.name)
                    .font(TextStyle::Heading)
                    .hint_text(format!("{} Beats", accent.beats.len())),
            );
        });

        let target_offset = total_width * 0.40;
        let used_width = ui.min_size().x;
        let spacer_width = (target_offset - used_width).max(0.0);
        ui.add_space(spacer_width);

        ui.horizontal(|ui| {
            ui.label("Subdivision:");

            for value in [1, 2, 3, 4, 6, 8] {
                ui.selectable_value(&mut accent.subdivision, value, value.to_string());
            }
        });

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            if icon_button(
                ui,
                egui::include_image!("../../../../assets/icons/trash.png"),
                SMALL_ICON_SIZE,
            ) {
                println!("Trash clicked!");
                *action = Some(AccentAction::Delete(accent_index));
            }
        });
    });
}

#[allow(clippy::too_many_arguments)]
fn draw_accent_beats(
    ui: &mut Ui,
    accent: &mut AccentData,
    accent_index: usize,
    menu_state: &mut u32,
    playing: bool,
    current_click: Option<(usize, usize)>,
    colors: BeatColors,
    action: &mut Option<AccentAction>,
) {
    ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            if icon_button(
                ui,
                egui::include_image!("../../../../assets/icons/down.png"),
                SMALL_ICON_SIZE,
            ) {
                println!("Move Accent Down Clicked!");
                *action = Some(AccentAction::MoveDown(accent_index));
            }

            let old_spacing = ui.spacing().item_spacing;
            ui.spacing_mut().item_spacing.y = 0.0;

            for (beat_index, beat) in accent.beats.iter_mut().enumerate() {
                draw_beat_column(
                    ui,
                    beat,
                    accent_index,
                    beat_index,
                    menu_state,
                    playing,
                    current_click,
                    colors,
                );
            }

            ui.spacing_mut().item_spacing = old_spacing;
        });

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            draw_beat_count_controls(ui, accent, colors);
        });
    });
}

#[allow(clippy::too_many_arguments)]
fn draw_beat_column(
    ui: &mut Ui,
    beat: &mut BeatData,
    accent_index: usize,
    beat_index: usize,
    menu_state: &mut u32,
    playing: bool,
    current_click: Option<(usize, usize)>,
    colors: BeatColors,
) {
    let button_size = Vec2::new(BEAT_BUTTON_WIDTH, BEAT_BUTTON_HEIGHT);
    let start_pos = ui.cursor().min;
    let menu_state_id = beat_menu_state(accent_index, beat_index);

    let mut cursor = start_pos;

    let states = beat_state_rows(colors);

    let invisible_frame = Frame {
        fill: Color32::TRANSPARENT,
        stroke: Stroke::NONE,
        corner_radius: CornerRadius::ZERO,
        outer_margin: egui::Margin::default(),
        inner_margin: egui::Margin::default(),
        ..Default::default()
    };

    let response = invisible_frame.show(ui, |ui| {
        for (state, color, rounding) in states {
            let rect = Rect::from_min_size(cursor, button_size);
            let response = ui.allocate_rect(rect, Sense::click());

            if response.hovered() {
                *menu_state = menu_state_id;
            }

            let selected_color = selected_beat_color(beat.state, colors);

            let fill = if current_click == Some((accent_index, beat_index)) && playing {
                colors.override_color
            } else if *menu_state != menu_state_id {
                selected_color
            } else {
                color
            };

            ui.painter()
                .rect(rect, rounding, fill, Stroke::NONE, egui::StrokeKind::Middle);

            if response.clicked() {
                beat.state = state;
            }

            cursor.y += button_size.y;
        }

        draw_beat_number(ui, start_pos, button_size, states.len(), beat_index);
    });

    if !response.response.hovered() && *menu_state == menu_state_id {
        *menu_state = 0;
    }
}

fn draw_beat_number(
    ui: &mut Ui,
    start_pos: egui::Pos2,
    button_size: Vec2,
    state_count: usize,
    beat_index: usize,
) {
    let total_height = button_size.y * state_count as f32;
    let full_rect = Rect::from_min_size(start_pos, Vec2::new(button_size.x, total_height));

    ui.painter().text(
        full_rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{}", beat_index + 1),
        TextStyle::Button.resolve(ui.style()),
        Color32::WHITE,
    );
}

fn draw_beat_count_controls(ui: &mut Ui, accent: &mut AccentData, colors: BeatColors) {
    Frame::group(ui.style())
        .corner_radius(5)
        .fill(colors.weak)
        .show(ui, |ui| {
            let size = [TINY_ICON_SIZE, TINY_ICON_SIZE];

            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        size,
                        ImageButton::new(egui::include_image!(
                            "../../../../assets/icons/minus.png"
                        ))
                        .frame(false),
                    )
                    .clicked()
                    && accent.beats.len() > 1
                {
                    println!("Minus clicked!");
                    accent.beats.pop();
                }

                if ui
                    .add_sized(
                        size,
                        ImageButton::new(egui::include_image!("../../../../assets/icons/plus.png"))
                            .frame(false),
                    )
                    .clicked()
                {
                    println!("Plus clicked!");

                    accent.beats.push(BeatData {
                        state: BeatState::Strong,
                    });
                }
            });
        });
}

fn apply_accent_action(app: &mut AppData, action: AccentAction) {
    let accents = &mut app.parameters.accents.accents;

    match action {
        AccentAction::MoveUp(index) => {
            if index > 0 && index < accents.len() {
                accents.swap(index, index - 1);
            }
        }

        AccentAction::MoveDown(index) => {
            if index + 1 < accents.len() {
                accents.swap(index, index + 1);
            }
        }

        AccentAction::Duplicate(index) => {
            if index < accents.len() {
                accents.insert(index, accents[index].clone());
            }
        }

        AccentAction::Insert(index) => {
            if index <= accents.len() {
                accents.insert(index, default_accent());
            }
        }

        AccentAction::Delete(index) => {
            if index < accents.len() && accents.len() > 1 {
                accents.remove(index);
            }
        }
    }
}

fn default_accent() -> AccentData {
    AccentData {
        beats: vec![BeatData {
            state: BeatState::Downbeat,
        }],
        subdivision: 1,
        name: String::new(),
    }
}

fn beat_colors(app: &AppData) -> BeatColors {
    let scheme = &app.settings.color_scheme;

    BeatColors {
        override_color: Color32::from_hex(&scheme.override_color).unwrap(),
        downbeat: Color32::from_hex(&scheme.downbeat_color).unwrap(),
        strong: Color32::from_hex(&scheme.strong_color).unwrap(),
        weak: Color32::from_hex(&scheme.weak_color).unwrap(),
        off: Color32::from_hex(&scheme.off_color).unwrap(),
    }
}

fn beat_state_rows(colors: BeatColors) -> [(BeatState, Color32, CornerRadius); 4] {
    let radius = 2;

    let rounding_top = CornerRadius {
        nw: radius,
        ne: radius,
        sw: 0,
        se: 0,
    };

    let rounding_mid = CornerRadius::ZERO;

    let rounding_bot = CornerRadius {
        nw: 0,
        ne: 0,
        sw: radius,
        se: radius,
    };

    [
        (BeatState::Downbeat, colors.downbeat, rounding_top),
        (BeatState::Strong, colors.strong, rounding_mid),
        (BeatState::Weak, colors.weak, rounding_mid),
        (BeatState::Off, colors.off, rounding_bot),
    ]
}

fn selected_beat_color(state: BeatState, colors: BeatColors) -> Color32 {
    match state {
        BeatState::Downbeat => colors.downbeat,
        BeatState::Strong => colors.strong,
        BeatState::Weak => colors.weak,
        BeatState::Off => colors.off,
    }
}

fn beat_menu_state(accent_index: usize, beat_index: usize) -> u32 {
    ((accent_index * BEAT_MENU_STATE_MULTIPLIER) + beat_index + 1) as u32
}

fn icon_button(ui: &mut Ui, image: egui::ImageSource<'static>, size: f32) -> bool {
    ui.add_sized([size, size], ImageButton::new(image).frame(false))
        .clicked()
}

fn truncate(s: &str, max_chars: usize) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if i >= max_chars {
            result.push('…');
            break;
        }

        result.push(c);
    }

    result
}

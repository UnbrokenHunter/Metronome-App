use crate::app::logic::load_utils::Loadable;
use crate::app::systems::colors::theme_presets::Theme;
use egui::{Context, Id};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static THEMES: Lazy<Themes> = Lazy::new(|| {
    let themes = Themes::load();

    println!("Loaded Themes:");
    themes
        .0
        .iter()
        .enumerate()
        .for_each(|(i, f)| println!("  - {}: {}", i, f.name));

    themes
});

#[derive(Serialize, Deserialize)]
struct Themes(Vec<Theme>);

impl Themes {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn get(&self, i: usize) -> Option<&Theme> {
        self.0.get(i)
    }
}

impl Loadable for Themes {
    const PATH: &str = "assets/config/theme.json";

    fn default_value() -> Self {
        Self(vec![
            Theme::dark(),
            Theme::light(),
            Theme::pastel(),
            Theme::nord(),
            Theme::high_contrast(),
        ])
    }
}

// ------------ Public API -------------

fn selected_id() -> Id {
    Id::new("core.theme.selected_index")
}

fn clamp_index(idx: usize) -> usize {
    let n = THEMES.len();
    if n == 0 { 0 } else { idx.min(n - 1) }
}

/// Get the currently selected theme as a concrete ThemeBuffered.
///
/// Behavior:
/// - If no persisted index is stored yet, this will create it with `0` and persist it.
/// - Always clamps to a valid index.
/// - Panics if there are zero themes (shouldn’t happen given your defaults).
pub fn theme(ctx: &Context) -> Theme {
    let idx = ctx.data_mut(|d| {
        // If missing, write 0 and return 0
        if let Some(saved) = d.get_persisted::<usize>(selected_id()) {
            saved
        } else {
            d.insert_persisted(selected_id(), 0usize);
            0
        }
    });

    let idx = clamp_index(idx);
    THEMES.get(idx).expect("at least one theme present").clone()
}

/// Persist a new selected theme index.
///
/// - Clamps to valid range.
/// - Stores in egui’s persisted memory immediately (so it survives restarts).
pub fn select_theme(ctx: &Context, idx: usize) {
    let clamped = clamp_index(idx);
    ctx.data_mut(|d| d.insert_persisted(selected_id(), clamped));
}

/// Get all available themes as immutable refs to their serialized form.
///
/// These are the same objects `theme()` and `select_theme()` work with.
pub fn all_themes() -> &'static [Theme] {
    &THEMES.0
}

use serde::{Deserialize, Serialize};
use crate::app::AppData;
use crate::app::systems::colors::theme_presets::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppThemeData(pub Vec<Theme>);

impl AppThemeData {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&Theme> {
        self.0.get(index)
    }

    pub fn all(&self) -> &[Theme] {
        &self.0
    }
}

impl AppData {
    pub fn current_theme(&self) -> Theme {
        crate::app::systems::colors::themes::theme(
            &self.themes,
            self.settings.selected_theme_index,
        )
    }
}
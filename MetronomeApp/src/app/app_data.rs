use crate::app::data::{
    AppAccentPresetData, AppPracticeData, AppRunningData, AppSaveData, AppSettingsData,
};

pub struct AppData {
    pub parameters: AppSaveData,
    pub runtime: AppRunningData,
    pub settings: AppSettingsData,
    pub practice: AppPracticeData,
    pub accent_presets: AppAccentPresetData,
}

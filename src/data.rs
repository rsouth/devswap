use crate::config;
use crate::config::Settings;
use druid::Data;
use druid::Lens;
use std::sync::Arc;

#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    settings: config::Settings,
    display_window_x: f64,
    display_window_y: f64,
    window_visible: bool,
    current_text: Arc<String>,
    pub(crate) command_text: String,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            settings: Settings::default(),
            display_window_x: 0.0,
            display_window_y: 0.0,
            window_visible: true,
            current_text: Arc::new("".to_string()),
            command_text: "".to_string(),
        }
    }
}

impl AppState {
    pub(crate) fn toggle_window(&mut self) -> bool {
        self.window_visible = !self.window_visible;
        self.window_visible
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn get_settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn replace_settings(&mut self, settings: Settings) {
        self.settings = settings;
    }
}

use crate::config;
use crate::config::Settings;
use druid::Lens;
use druid::{Data};

#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    settings: config::Settings,
    display_window_x: f64,
    display_window_y: f64,
    window_visible: bool,
    pub(crate) current_text: String,
    pub(crate) command_text: String,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            settings: Settings::default(),
            display_window_x: 0.0,
            display_window_y: 0.0,
            window_visible: true,
            current_text: "".to_string(),
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

    pub fn add_to_command_history(&mut self, cmd: &str) {
        self.settings.add_to_command_history(cmd);
    }

    pub fn push_doc(&mut self, doc: &str) {
        println!("push docs: {}", doc);
        self.settings.push_doc(doc.to_string());
        self.current_text.clear();
        self.current_text.push_str(doc);
        self.current_text = doc.to_string();
    }

    pub fn pop_doc(&mut self) -> String {
        self.settings.pop_doc()
    }
}

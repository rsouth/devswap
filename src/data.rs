use crate::config;
use crate::config::Settings;
use crate::document::{DocType, Header};
use druid::Data;
use druid::Lens;

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

    pub fn push_doc(&mut self, doc: &Header) {
        println!("push docs: {:?}", &doc);
        if let DocType::FileBased(_) = doc.doc_type {
            self.settings.push_doc(doc);
        }

        self.switch_doc(doc);
    }

    fn switch_doc(&mut self, header: &Header) {
        match &header.doc_type {
            DocType::FileBased(file_name) => {
                println!("Load file {} here...", &file_name);
            }
            DocType::Hardcoded(content) => {
                println!("Switching to hardcoded content here...");
                self.current_text.clear();
                self.current_text = content.clone();
            }
            DocType::Empty => {
                self.current_text.clear();
            }
        }
    }

    pub fn pop_doc(&mut self) {
        match self.settings.pop_doc() {
            None => {
                self.current_text.clear();
            }
            Some(new_doc) => {
                self.switch_doc(&new_doc);
            }
        }
    }
}

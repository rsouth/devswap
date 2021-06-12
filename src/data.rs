use crate::config;
use crate::config::Settings;
use druid::Data;
use druid::Lens;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone, Data, Lens)]
pub struct AppData {
    pub settings: Rc<config::Settings>,
    display_window_x: f64,
    display_window_y: f64,
    window_visible: bool,
    current_text: Arc<String>,
    pub(crate) command_text: String,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            settings: Rc::new(Settings::default()),
            display_window_x: 0.0,
            display_window_y: 0.0,
            window_visible: true,
            current_text: Arc::new("".to_string()),
            command_text: "".to_string(),
        }
    }
}

impl AppData {
    pub(crate) fn toggle_window(&mut self) -> bool {
        self.window_visible = !self.window_visible;
        self.window_visible
    }
}

use crate::command::{Executable, ExecutionError};
use crate::config::Settings;
use crate::data;
use crate::data::AppState;

pub(crate) struct ShowHelp<'r> {
    app_data: &'r AppState,
}

impl<'r> ShowHelp<'r> {
    pub fn new(app_data: &'r mut data::AppState) -> Self {
        ShowHelp { app_data }
    }
}

impl Executable for ShowHelp<'_> {
    fn execute(&mut self) -> Result<u128, ExecutionError> {
        println!("Command: Showing Help");

        // Settings::save(self.app_data.get_settings());
        Ok(0)
    }
}

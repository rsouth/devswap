use crate::command::{Executable, ExecutionError};
use crate::config::Settings;
use crate::data;
use crate::data::AppState;

pub(crate) struct SaveSettings<'r> {
    app_data: &'r AppState,
}

impl<'r> SaveSettings<'r> {
    pub fn new(app_data: &'r mut data::AppState) -> Self {
        SaveSettings { app_data }
    }
}

impl Executable for SaveSettings<'_> {
    fn execute(&mut self) -> Result<u128, ExecutionError> {
        println!("Command: Saving Settings");

        Settings::save(self.app_data.get_settings());
        Ok(0)
    }
}

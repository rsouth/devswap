use crate::command::{Executable, ExecutionError};
use crate::data;
use crate::data::AppState;
use std::path::Path;

pub(crate) struct WriteFile<'r> {
    app_data: &'r mut AppState,
}

impl<'r> WriteFile<'r> {
    pub fn new(app_data: &'r mut data::AppState) -> Self {
        WriteFile { app_data }
    }
}

impl Executable for WriteFile<'_> {
    fn execute(&mut self) -> Result<u128, ExecutionError> {
        println!("Command: Saving File");
        match self.app_data.get_settings().current_doc_filename() {
            Some(h) => {
                println!("write the file here");
                let content = self.app_data.current_text.as_str();
                let p = Path::new(&h);
                std::fs::write(p, content).expect("failed to save settings");
            }
            _ => {
                println!("No filename for topdoc.. prob not file-based? or sth.")
            }
        };

        Ok(0)
    }
}

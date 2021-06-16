use crate::command::{Executable, ExecutionError};
use crate::data;
use crate::data::AppState;
use crate::document::{DocType, Header};

#[rustfmt::skip]
lazy_static! {
    static ref HELP_TEXT: String = 
    "\t\u{1f680} /dev/swap - swapfile for your noggin\n\
     \tversion 2021.06.15\n\n\
       What's all this then?".to_string();
}

pub(crate) struct ShowHelp<'r> {
    app_data: &'r mut AppState,
}

impl<'r> ShowHelp<'r> {
    pub fn new(app_data: &'r mut data::AppState) -> Self {
        ShowHelp { app_data }
    }

    fn halp() -> Header {
        Header::new(DocType::Hardcoded(HELP_TEXT.to_string()))
    }
}

impl Executable for ShowHelp<'_> {
    fn execute(&mut self) -> Result<u128, ExecutionError> {
        println!("Command: Showing Help");
        self.app_data.push_doc(&ShowHelp::halp());
        Ok(0)
    }
}

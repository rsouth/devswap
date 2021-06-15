use druid::DelegateCtx;

use crate::command::{Executable, ExecutionError};
use crate::data;
use crate::data::AppState;

pub(crate) struct InsertMode<'r> {
    _app_data: &'r AppState,
    _ctx: &'r DelegateCtx<'r>,
}

impl<'r> InsertMode<'r> {
    pub fn new(app_data: &'r mut data::AppState, ctx: &'r druid::DelegateCtx<'r>) -> Self {
        InsertMode {
            _app_data: app_data,
            _ctx: ctx,
        }
    }
}

impl Executable for InsertMode<'_> {
    fn execute(&mut self) -> Result<u128, ExecutionError> {
        println!("Command: Switching to Insert Mode");
        Ok(0)
    }
}
